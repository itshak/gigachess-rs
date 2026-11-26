| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.975x | 1.395x |
| materialize | 0.998x | 0.994x | 1.500x |
| perft_bulk_d4 | 0.992x | 0.983x | 1.203x |
| perft_full_d3 | 1.006x | 0.955x | 1.474x |
| fen_parse | 1.018x | 1.008x | 1.074x |
| fen_write_native_ep | 0.997x | 0.961x | 1.308x |
| snapshot | 0.914x | 0.910x | 1.154x |
| san_parse_all | 0.997x | 0.987x | 1.901x |
| san_render_native | 1.021x | 1.015x | 2.129x |
| apply_children_full | 1.018x | 1.010x | 1.150x |
| make_unmake_full | 0.997x | 0.981x | 1.148x |
| startpos | 0.996x | 0.996x | 1.292x |
| chess960_materialize | 1.001x | 1.000x | 1.445x |
| chess960_perft_d3 | 0.998x | 0.992x | 1.378x |
| polyglot_key | 0.999x | 0.997x | 39.990x |
| polyglot_recompute | 1.002x | 1.000x | 0.978x |
| repetition_key | 1.028x | 0.983x | 20.242x |
| corpus_count | 1.000x | 0.999x | 1.324x |
| corpus_materialize | 1.012x | 1.008x | 1.086x |
| corpus_uci_sorted | 0.988x | 0.982x | 1.539x |
| replay_trusted_native | 1.009x | 1.005x | 0.712x |
| replay_checked_native | 1.019x | 1.009x | 1.064x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.97x).
