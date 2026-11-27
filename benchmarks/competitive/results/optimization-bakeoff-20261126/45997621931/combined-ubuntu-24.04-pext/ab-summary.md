| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.992x | 0.960x | 1.551x |
| materialize | 0.995x | 0.987x | 1.594x |
| perft_bulk_d4 | 1.008x | 0.996x | 1.198x |
| perft_full_d3 | 1.018x | 0.997x | 1.371x |
| fen_parse | 0.941x | 0.830x | 1.096x |
| fen_write_native_ep | 1.009x | 0.972x | 2.065x |
| snapshot | 1.001x | 0.999x | 1.001x |
| san_parse_all | 1.026x | 0.997x | 1.892x |
| san_render_native | 0.997x | 0.983x | 2.123x |
| apply_children_full | 0.999x | 0.981x | 1.354x |
| make_unmake_full | 0.919x | 0.870x | 1.093x |
| startpos | 0.892x | 0.892x | 1.001x |
| chess960_materialize | 1.034x | 1.014x | 1.416x |
| chess960_perft_d3 | 0.995x | 0.987x | 1.465x |
| polyglot_key | 1.008x | 1.002x | 39.318x |
| polyglot_recompute | 1.014x | 1.008x | 1.017x |
| repetition_key | 0.818x | 0.696x | 19.054x |
| corpus_count | 0.942x | 0.897x | 1.059x |
| corpus_materialize | 1.014x | 1.007x | 1.090x |
| corpus_uci_sorted | 1.020x | 1.016x | 1.571x |
| replay_trusted_native | 0.902x | 0.827x | 0.799x |
| replay_checked_native | 0.969x | 0.967x | 1.120x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.09x).
