| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.987x | 1.209x |
| materialize | 1.007x | 1.000x | 1.255x |
| perft_bulk_d4 | 1.013x | 0.993x | 1.222x |
| perft_full_d3 | 1.008x | 0.995x | 1.356x |
| fen_parse | 1.005x | 0.999x | 1.194x |
| fen_write_native_ep | 0.992x | 0.980x | 1.773x |
| snapshot | 0.999x | 0.997x | 0.992x |
| san_parse_all | 1.001x | 0.997x | 2.015x |
| san_render_native | 0.967x | 0.937x | 2.299x |
| apply_children_full | 1.017x | 0.988x | 1.163x |
| make_unmake_full | 1.021x | 1.008x | 1.325x |
| startpos | 1.032x | 1.032x | 1.304x |
| chess960_materialize | 0.984x | 0.965x | 1.328x |
| chess960_perft_d3 | 0.976x | 0.976x | 1.368x |
| polyglot_key | 1.084x | 0.995x | 27.615x |
| polyglot_recompute | 0.998x | 0.995x | 1.024x |
| repetition_key | 1.101x | 1.006x | 21.123x |
| corpus_count | 1.009x | 1.008x | 1.317x |
| corpus_materialize | 1.019x | 1.018x | 1.201x |
| corpus_uci_sorted | 1.000x | 0.987x | 1.727x |
| replay_trusted_native | 1.030x | 1.009x | 0.718x |
| replay_checked_native | 1.013x | 1.009x | 1.058x |

Cold first legal move list: 0.015 ms -> 0.015 ms (1.02x).
