use super::helpers::emit_stage;
use super::models::{extract_youtube_video_id, RestreamerInfo, RestreamerSearchCacheEntry};
use crate::error::CliptzyError;
use crate::orchestrator::job_cache::{cache_file, hash_payload, read_json_cache, write_json_cache};
use crate::orchestrator::pipeline::PipelineContext;
use crate::utils::date::{add_days_yyyymmdd, is_upload_within_reaction_window};
use crate::video::youtube::VideoAnalysisResult;
use std::collections::HashSet;

const MAX_SEARCH_QUERIES: usize = 8;
const TARGET_RESULT_COUNT: usize = 10;

pub(crate) fn migrate_cached_restreamers(
    cached: &RestreamerSearchCacheEntry,
) -> Vec<RestreamerInfo> {
    if !cached.restreamers.is_empty() {
        return cached.restreamers.clone();
    }

    cached
        .urls
        .iter()
        .map(|url| {
            let video_id = extract_youtube_video_id(url);
            RestreamerInfo {
                video_id: video_id.clone(),
                video_url: url.clone(),
                title: url.clone(),
                uploader: String::new(),
                thumbnail: format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", video_id),
                duration: 0.0,
                upload_date: None,
                view_count: None,
            }
        })
        .collect()
}

fn parse_restreamer_entry(entry: &serde_json::Value) -> Option<RestreamerInfo> {
    let id = entry.get("id").and_then(|i| i.as_str()).unwrap_or("");
    if id.is_empty() {
        return None;
    }

    let entry_url = entry
        .get("webpage_url")
        .and_then(|u| u.as_str())
        .or_else(|| entry.get("url").and_then(|u| u.as_str()))
        .unwrap_or("");
    let video_url = if entry_url.starts_with("http") {
        entry_url.to_string()
    } else {
        format!("https://www.youtube.com/watch?v={}", id)
    };

    let thumbnail = entry
        .get("thumbnail")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", id));

    Some(RestreamerInfo {
        video_id: id.to_string(),
        video_url,
        title: entry
            .get("title")
            .and_then(|t| t.as_str())
            .unwrap_or("Tanpa Judul")
            .to_string(),
        uploader: entry
            .get("uploader")
            .or_else(|| entry.get("channel"))
            .and_then(|u| u.as_str())
            .unwrap_or("")
            .to_string(),
        thumbnail,
        duration: entry
            .get("duration")
            .and_then(|d| d.as_f64())
            .unwrap_or(0.0),
        upload_date: entry
            .get("upload_date")
            .and_then(|d| d.as_str())
            .map(|s| s.to_string()),
        view_count: entry.get("view_count").and_then(|v| v.as_u64()),
    })
}

fn push_unique_query(queries: &mut Vec<String>, seen: &mut HashSet<String>, query: &str) {
    let trimmed = query.trim();
    if trimmed.len() < 3 {
        return;
    }
    let key = trimmed.to_lowercase();
    if seen.insert(key) {
        queries.push(trimmed.to_string());
    }
}

fn extract_team_token(side: &str) -> String {
    let mut cleaned = side
        .trim()
        .trim_matches(|c: char| c == '|' || c == '-' || c == ':');
    if let Some((_, right)) = cleaned.rsplit_once('-') {
        let right = right.trim();
        if !right.is_empty() && right.len() <= 24 {
            cleaned = right;
        }
    }
    if let Some((_, right)) = cleaned.rsplit_once('|') {
        let right = right.trim();
        if !right.is_empty() && right.len() <= 24 {
            cleaned = right;
        }
    }

    cleaned
        .split_whitespace()
        .filter(|w| {
            let lower = w.to_lowercase();
            !matches!(
                lower.as_str(),
                "match" | "game" | "week" | "day" | "hari" | "vs" | "v"
            )
        })
        .take(3)
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_vs_teams(title: &str) -> Option<(String, String)> {
    for sep in [" vs ", " VS ", " v ", " V ", " vs. ", " VS. "] {
        if let Some((left, right)) = title.split_once(sep) {
            let team_a = extract_team_token(left);
            let team_b = extract_team_token(right);
            if team_a.len() >= 2 && team_b.len() >= 2 {
                return Some((team_a, team_b));
            }
        }
    }
    None
}

fn extract_league_hint(title: &str) -> Option<String> {
    let upper = title.to_uppercase();
    for marker in ["MPL", "MSC", "M-Series", "M SERIES", "PMPL", "MDL"] {
        if let Some(idx) = upper.find(marker) {
            let snippet = title[idx..].trim();
            let hint = snippet
                .split_whitespace()
                .take(4)
                .collect::<Vec<_>>()
                .join(" ");
            if hint.len() >= 3 {
                return Some(hint);
            }
        }
    }
    None
}

fn build_heuristic_queries(title: &str, custom_keywords: Option<&str>) -> Vec<String> {
    let mut queries = Vec::new();
    let mut seen = HashSet::new();

    push_unique_query(&mut queries, &mut seen, title);

    if let Some(keywords) = custom_keywords {
        for part in keywords.split([',', ';', '|']) {
            push_unique_query(&mut queries, &mut seen, part);
        }
    }

    if let Some((team_a, team_b)) = extract_vs_teams(title) {
        push_unique_query(&mut queries, &mut seen, &format!("{team_a} {team_b} nobar"));
        push_unique_query(
            &mut queries,
            &mut seen,
            &format!("{team_a} {team_b} live reaction"),
        );
        push_unique_query(
            &mut queries,
            &mut seen,
            &format!("{team_a} {team_b} reaksi"),
        );
        push_unique_query(&mut queries, &mut seen, &format!("{team_a} {team_b} live"));
        push_unique_query(&mut queries, &mut seen, &format!("{team_a} nobar live"));
        push_unique_query(&mut queries, &mut seen, &format!("{team_b} nobar live"));
    }

    if let Some(league) = extract_league_hint(title) {
        push_unique_query(&mut queries, &mut seen, &format!("{league} nobar"));
        push_unique_query(&mut queries, &mut seen, &format!("{league} live reaction"));
    }

    queries
}

fn restreamer_search_prompt(video_info: &VideoAnalysisResult) -> String {
    format!(
        "Kamu membantu menemukan VOD livestream reaksi/nobar esports Mobile Legends di YouTube Indonesia.\n\
        Judul VOD resmi pertandingan: \"{}\"\n\
        Video ID: {}\n\
        Tanggal upload: {}\n\n\
        Restreamer sering memakai judul clickbait yang TIDAK menyebut kedua tim secara literal \
        (contoh: \"Nobar MPL ID S18\", \"Akankah AE wangi hari ini?\", \"RRQ kesurupan lagi\").\n\n\
        Buat 6 kueri pencarian YouTube yang pendek dan realistis untuk menemukan stream nobar/reaksi pertandingan ini.\n\
        Sertakan singkatan tim, petunjuk liga/musim (MPL/MSC/MDL), dan kata kunci seperti nobar, live reaction, reaksi.\n\n\
        Kembalikan HANYA array JSON string tanpa markdown. Contoh:\n\
        [\"AE RRQ nobar MPL\", \"Alter Ego RRQ live\", \"MPL S18 nobar\"]",
        video_info.title,
        video_info.video_id,
        video_info.upload_date.as_deref().unwrap_or("tidak diketahui")
    )
}

fn parse_ai_query_list(response: &str) -> Vec<String> {
    let trimmed = response.trim();
    let json_slice = if let Some(start) = trimmed.find('[') {
        if let Some(end) = trimmed.rfind(']') {
            &trimmed[start..=end]
        } else {
            return Vec::new();
        }
    } else {
        return Vec::new();
    };

    serde_json::from_str::<Vec<String>>(json_slice).unwrap_or_default()
}

async fn expand_queries_with_ai(
    ctx: &PipelineContext,
    video_info: &VideoAnalysisResult,
) -> Vec<String> {
    let config = &ctx.config.ai;
    let provider_ready = match config.provider.as_str() {
        "gemini" => !config.gemini_key.trim().is_empty(),
        "openai" => !config.openai_key.trim().is_empty(),
        "ollama" => !config.ollama_host.trim().is_empty(),
        _ => !config.gemini_key.trim().is_empty(),
    };

    if !provider_ready {
        return Vec::new();
    }

    let ai_provider = crate::ai::create_provider(config);
    let prompt = restreamer_search_prompt(video_info);

    match ai_provider.generate(&prompt, Some(&ctx.progress_tx)).await {
        Ok(response) => {
            let queries = parse_ai_query_list(&response);
            log::info!(
                "AI menghasilkan {} kueri pencarian restreamer tambahan.",
                queries.len()
            );
            queries
        }
        Err(e) => {
            log::warn!(
                "Gagal memperluas kueri pencarian restreamer via AI, memakai heuristik saja: {}",
                e
            );
            Vec::new()
        }
    }
}

async fn build_search_queries(
    ctx: &PipelineContext,
    video_info: &VideoAnalysisResult,
    custom_keywords: Option<&str>,
) -> Vec<String> {
    let mut queries = build_heuristic_queries(&video_info.title, custom_keywords);
    let mut seen: HashSet<String> = queries.iter().map(|q| q.to_lowercase()).collect();

    for ai_query in expand_queries_with_ai(ctx, video_info).await {
        push_unique_query(&mut queries, &mut seen, &ai_query);
    }

    queries.truncate(MAX_SEARCH_QUERIES);
    queries
}

fn queries_hash(queries: &[String]) -> String {
    hash_payload(&serde_json::to_string(queries).unwrap_or_default())
}

async fn run_single_yt_search(
    ctx: &PipelineContext,
    search_query: &str,
    cookies_path: &Option<String>,
) -> Result<Vec<RestreamerInfo>, CliptzyError> {
    let yt_search_query = format!("ytsearch40:'{}'", search_query);
    log::info!("Kueri yt-dlp: {}", yt_search_query);

    let mut cmd = tokio::process::Command::new(&ctx.deps.ytdlp);
    cmd.arg(&yt_search_query)
        .arg("--dump-single-json")
        .arg("--no-warnings")
        .arg("--extractor-args")
        .arg("youtube:player-client=android,web,default")
        .arg("--remote-components")
        .arg("ejs:github")
        .arg("--match-filter")
        .arg("duration > 3600");

    if let Some(browser) = cookies_path {
        if !browser.is_empty() {
            cmd.arg("--cookies-from-browser").arg(browser);
        }
    }

    let output = cmd.output().await.map_err(|e| {
        log::error!("[Compilation] Gagal spawn yt-dlp search: {}", e);
        CliptzyError::FFmpeg {
            code: -1,
            message: format!("Gagal spawn yt-dlp search: {}", e),
        }
    })?;

    if !output.status.success() {
        let err_str = String::from_utf8_lossy(&output.stderr);
        log::warn!(
            "[Compilation] yt-dlp search gagal untuk kueri '{}': {}",
            search_query,
            err_str
        );
        return Ok(Vec::new());
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let json_start = json_str.find('{').unwrap_or(0);
    let json_end = json_str.rfind('}').map(|i| i + 1).unwrap_or(json_str.len());
    let clean_json = if json_start < json_end {
        &json_str[json_start..json_end]
    } else {
        &json_str
    };

    let json_out: serde_json::Value = serde_json::from_str(clean_json).map_err(|e| {
        log::error!("[Compilation] Gagal parse output search JSON: {}", e);
        CliptzyError::Config(format!("Gagal parse output search JSON: {}", e))
    })?;

    let mut results = Vec::new();
    if let Some(entries) = json_out.get("entries").and_then(|e| e.as_array()) {
        for entry in entries {
            if let Some(info) = parse_restreamer_entry(entry) {
                results.push(info);
            }
        }
    }

    Ok(results)
}

fn merge_restreamer_results(
    target: &mut Vec<RestreamerInfo>,
    incoming: Vec<RestreamerInfo>,
    uploaders: &mut Vec<String>,
    seen_ids: &mut HashSet<String>,
    min_duration_sec: f64,
    main_upload_date: &Option<String>,
) {
    for info in incoming {
        let title = &info.title;
        let id = &info.video_id;
        let url = &info.video_url;
        let duration = info.duration;
        let entry_upload_date = info.upload_date.as_deref().unwrap_or("");

        let is_shorts_url = url.contains("/shorts/") || id.is_empty();
        let is_shorts_title = title.to_lowercase().contains("#shorts");

        let upload_date_ok = match (main_upload_date, entry_upload_date.is_empty()) {
            (Some(main_date), false) => {
                is_upload_within_reaction_window(entry_upload_date, main_date, 1)
            }
            (Some(_), true) => false,
            (None, _) => true,
        };

        if is_shorts_url
            || is_shorts_title
            || uploaders.contains(&info.uploader)
            || duration < min_duration_sec
            || !upload_date_ok
            || seen_ids.contains(id)
        {
            continue;
        }

        uploaders.push(info.uploader.clone());
        seen_ids.insert(id.clone());
        target.push(info);
    }
}

pub async fn search_restreamers(
    ctx: &PipelineContext,
    video_info: &VideoAnalysisResult,
    custom_keywords: Option<String>,
    min_duration_minutes: Option<u32>,
) -> Result<Vec<RestreamerInfo>, CliptzyError> {
    let original_title = video_info.title.clone();
    let main_upload_date = video_info.upload_date.clone();

    log::info!(
        "Memulai Pencarian Restreamer / Reaksi (Phase 4) untuk judul: {}",
        original_title
    );

    emit_stage(
        ctx,
        "search",
        "Menyusun kueri pencarian restreamer...",
        72,
        100,
    );

    let cookies_path = ctx.config.browser.as_deref().map(|s| s.to_string());
    let min_duration = min_duration_minutes.unwrap_or(20);
    let search_cache_path = cache_file(&ctx.job_dir, "restreamer_search.json");
    let custom_kw = custom_keywords.as_deref().filter(|s| !s.trim().is_empty());

    let search_queries = build_search_queries(ctx, video_info, custom_kw).await;
    let current_queries_hash = queries_hash(&search_queries);

    log::info!(
        "Kueri pencarian restreamer ({}): {:?}",
        search_queries.len(),
        search_queries
    );

    if let Some(cached) = read_json_cache::<RestreamerSearchCacheEntry>(&search_cache_path) {
        let cache_hash_ok = if cached.queries_hash.is_empty() {
            cached.query == original_title
        } else {
            cached.queries_hash == current_queries_hash
        };

        if cache_hash_ok
            && cached.min_duration_minutes == min_duration
            && cached.main_upload_date == main_upload_date
        {
            let restreamers = migrate_cached_restreamers(&cached);
            if !restreamers.is_empty() {
                log::info!(
                    "Menggunakan hasil pencarian restreamer dari cache ({} item): {:?}",
                    restreamers.len(),
                    search_cache_path
                );
                emit_stage(
                    ctx,
                    "search",
                    &format!(
                        "Menggunakan cache pencarian ({} restreamer)...",
                        restreamers.len()
                    ),
                    90,
                    100,
                );
                return Ok(restreamers);
            }
        }

        log::info!("Cache pencarian restreamer tidak valid (kueri berubah), mencari ulang...");
    }

    if let Some(ref upload_date) = main_upload_date {
        log::info!(
            "Filter tanggal upload restreamer: {} hingga +1 hari dari {}",
            upload_date,
            add_days_yyyymmdd(upload_date, 1).unwrap_or_else(|| "?".to_string())
        );
    }

    let min_duration_sec = min_duration as f64 * 60.0;
    let mut potential_restreamers: Vec<RestreamerInfo> = Vec::new();
    let mut uploaders: Vec<String> = Vec::new();
    let mut seen_ids: HashSet<String> = HashSet::new();

    for (idx, query) in search_queries.iter().enumerate() {
        emit_stage(
            ctx,
            "search",
            &format!(
                "Mencari restreamer ({}/{}) — {}...",
                idx + 1,
                search_queries.len(),
                query
            ),
            75 + (idx as u32 * 10 / search_queries.len().max(1) as u32),
            100,
        );

        let batch = run_single_yt_search(ctx, query, &cookies_path).await?;
        merge_restreamer_results(
            &mut potential_restreamers,
            batch,
            &mut uploaders,
            &mut seen_ids,
            min_duration_sec,
            &main_upload_date,
        );

        if potential_restreamers.len() >= TARGET_RESULT_COUNT {
            log::info!(
                "Target {} restreamer tercapai, menghentikan pencarian tambahan.",
                TARGET_RESULT_COUNT
            );
            break;
        }
    }

    potential_restreamers.sort_by(|a, b| a.video_id.cmp(&b.video_id));
    potential_restreamers.dedup_by(|a, b| a.video_id == b.video_id);

    if potential_restreamers.is_empty() {
        log::warn!(
            "[Compilation] Tidak ada restreamer VOD ditemukan untuk kueri: {:?}",
            search_queries
        );
    } else {
        log::info!(
            "Ditemukan {} URL Restreamer VOD berpotensi yang relevan.",
            potential_restreamers.len()
        );
    }

    emit_stage(
        ctx,
        "search",
        &format!(
            "Ditemukan {} restreamer potensial.",
            potential_restreamers.len()
        ),
        90,
        100,
    );

    write_json_cache(
        &search_cache_path,
        &RestreamerSearchCacheEntry {
            query: original_title,
            queries_hash: current_queries_hash,
            min_duration_minutes: min_duration,
            main_upload_date,
            restreamers: potential_restreamers.clone(),
            urls: Vec::new(),
        },
    )?;

    Ok(potential_restreamers)
}
