| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.992x | 1.083x |
| materialize | 1.001x | 0.998x | 1.169x |
| perft_bulk_d4 | 1.014x | 1.005x | 0.900x |
| perft_full_d3 | 0.998x | 0.995x | 0.611x |
| fen_parse | 1.001x | 0.998x | 1.201x |
| fen_write_native_ep | 1.008x | 1.004x | 1.078x |
| snapshot | 0.984x | 0.977x | 0.755x |
| san_parse_all | 1.851x | 1.779x | 1.703x |
| san_render_native | 1.000x | 0.993x | 1.159x |
| apply_children_full | 1.008x | 1.004x | 0.656x |
| make_unmake_full | 1.002x | 1.001x | 0.892x |
| startpos | 0.987x | 0.987x | 0.763x |
| chess960_materialize | 0.997x | 0.994x | 1.101x |
| chess960_perft_d3 | 0.991x | 0.987x | 0.830x |
| polyglot_key | 0.971x | 0.954x | 22.492x |
| polyglot_recompute | 1.004x | 1.001x | 1.133x |
| repetition_key | 1.059x | 0.997x | 14.665x |
| corpus_count | 0.989x | 0.974x | 1.181x |
| corpus_materialize | 0.990x | 0.986x | 1.101x |
| corpus_uci_sorted | 0.994x | 0.992x | 0.569x |
| replay_trusted_native | 0.966x | 0.945x | 0.316x |
| replay_checked_native | 0.994x | 0.994x | 0.655x |

Cold first legal move list: 154.079 ms → 154.184 ms (1.00x).
