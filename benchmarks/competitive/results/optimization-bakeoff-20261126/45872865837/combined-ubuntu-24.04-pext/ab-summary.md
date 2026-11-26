| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.980x | 1.557x |
| materialize | 0.927x | 0.894x | 1.503x |
| perft_bulk_d4 | 1.012x | 0.996x | 1.202x |
| perft_full_d3 | 1.235x | 1.064x | 1.652x |
| fen_parse | 0.943x | 0.844x | 1.122x |
| fen_write_native_ep | 0.969x | 0.927x | 1.982x |
| snapshot | 1.002x | 0.995x | 0.997x |
| san_parse_all | 1.019x | 0.988x | 2.006x |
| san_render_native | 0.998x | 0.993x | 2.072x |
| apply_children_full | 0.965x | 0.938x | 1.307x |
| make_unmake_full | 0.929x | 0.894x | 1.105x |
| startpos | 1.001x | 1.001x | 0.997x |
| chess960_materialize | 0.936x | 0.924x | 1.399x |
| chess960_perft_d3 | 0.995x | 0.993x | 1.467x |
| polyglot_key | 1.008x | 0.997x | 39.814x |
| polyglot_recompute | 1.020x | 1.017x | 1.020x |
| repetition_key | 0.996x | 0.989x | 23.776x |
| corpus_count | 0.939x | 0.892x | 1.062x |
| corpus_materialize | 0.996x | 0.995x | 1.072x |
| corpus_uci_sorted | 1.005x | 1.004x | 1.532x |
| replay_trusted_native | 0.979x | 0.970x | 0.874x |
| replay_checked_native | 1.002x | 0.985x | 1.113x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.05x).
