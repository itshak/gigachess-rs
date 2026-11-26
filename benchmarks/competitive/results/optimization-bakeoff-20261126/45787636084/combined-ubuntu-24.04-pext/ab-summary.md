| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.950x | 0.912x | 1.412x |
| materialize | 0.971x | 0.919x | 1.516x |
| perft_bulk_d4 | 0.830x | 0.755x | 1.057x |
| perft_full_d3 | 0.944x | 0.878x | 1.453x |
| fen_parse | 1.015x | 0.999x | 1.135x |
| fen_write_native_ep | 0.997x | 0.979x | 1.910x |
| snapshot | 1.818x | 1.803x | 0.997x |
| san_parse_all | 1.123x | 1.087x | 1.890x |
| san_render_native | 0.988x | 0.982x | 2.137x |
| apply_children_full | 1.040x | 0.995x | 1.093x |
| make_unmake_full | 1.002x | 0.979x | 1.141x |
| startpos | 1.199x | 1.199x | 0.998x |
| chess960_materialize | 0.999x | 0.989x | 1.426x |
| chess960_perft_d3 | 0.987x | 0.965x | 1.377x |
| polyglot_key | 1.040x | 0.992x | 39.855x |
| polyglot_recompute | 1.008x | 1.006x | 1.058x |
| repetition_key | 0.984x | 0.767x | 18.979x |
| corpus_count | 0.674x | 0.598x | 0.951x |
| corpus_materialize | 0.946x | 0.931x | 1.068x |
| corpus_uci_sorted | 0.977x | 0.971x | 1.559x |
| replay_trusted_native | 1.005x | 0.993x | 0.708x |
| replay_checked_native | 1.024x | 1.024x | 1.098x |

Cold first legal move list: 0.008 ms -> 0.006 ms (1.47x).
