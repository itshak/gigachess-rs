| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.995x | 1.164x |
| materialize | 1.009x | 0.995x | 1.240x |
| perft_bulk_d4 | 1.005x | 0.988x | 1.189x |
| perft_full_d3 | 0.998x | 0.931x | 1.124x |
| fen_parse | 0.861x | 0.824x | 1.087x |
| fen_write_native_ep | 0.941x | 0.934x | 1.196x |
| snapshot | 1.013x | 0.999x | 0.932x |
| san_parse_all | 0.996x | 0.987x | 2.172x |
| san_render_native | 0.964x | 0.932x | 2.375x |
| apply_children_full | 1.006x | 0.985x | 1.102x |
| make_unmake_full | 0.988x | 0.966x | 1.289x |
| startpos | 1.000x | 1.000x | 1.312x |
| chess960_materialize | 1.010x | 1.010x | 1.451x |
| chess960_perft_d3 | 0.998x | 0.998x | 1.414x |
| polyglot_key | 0.989x | 0.981x | 21.559x |
| polyglot_recompute | 0.996x | 0.991x | 0.976x |
| repetition_key | 0.998x | 0.995x | 20.451x |
| corpus_count | 1.000x | 0.999x | 1.233x |
| corpus_materialize | 0.993x | 0.992x | 1.350x |
| corpus_uci_sorted | 0.988x | 0.984x | 1.565x |
| replay_trusted_native | 1.060x | 0.997x | 0.720x |
| replay_checked_native | 1.041x | 1.030x | 1.040x |

Cold first legal move list: 0.012 ms -> 0.011 ms (1.08x).
