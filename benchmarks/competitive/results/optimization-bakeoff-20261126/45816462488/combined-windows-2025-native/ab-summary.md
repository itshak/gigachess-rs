| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.007x | 0.989x | 1.366x |
| materialize | 0.957x | 0.860x | 1.028x |
| perft_bulk_d4 | 1.008x | 1.002x | 1.328x |
| perft_full_d3 | 0.971x | 0.912x | 1.047x |
| fen_parse | 0.828x | 0.799x | 1.212x |
| fen_write_native_ep | 0.997x | 0.957x | 1.584x |
| snapshot | 1.002x | 0.996x | 1.161x |
| san_parse_all | 1.030x | 0.959x | 1.991x |
| san_render_native | 0.988x | 0.907x | 3.481x |
| apply_children_full | 0.995x | 0.940x | 1.105x |
| make_unmake_full | 1.042x | 0.972x | 1.317x |
| startpos | 1.004x | 1.004x | 1.196x |
| chess960_materialize | 1.066x | 1.015x | 1.260x |
| chess960_perft_d3 | 1.019x | 0.979x | 1.587x |
| polyglot_key | 0.999x | 0.997x | 20.318x |
| polyglot_recompute | 1.014x | 1.010x | 0.993x |
| repetition_key | 0.991x | 0.971x | 15.491x |
| corpus_count | 1.105x | 1.078x | 1.182x |
| corpus_materialize | 0.997x | 0.995x | 1.133x |
| corpus_uci_sorted | 1.064x | 1.058x | 1.489x |
| replay_trusted_native | 0.997x | 0.996x | 0.828x |
| replay_checked_native | 1.028x | 1.020x | 0.952x |

Cold first legal move list: 0.018 ms -> 0.016 ms (1.13x).
