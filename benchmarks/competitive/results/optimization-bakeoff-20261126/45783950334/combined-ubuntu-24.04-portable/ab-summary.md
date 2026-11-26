| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.992x | 1.166x |
| materialize | 1.008x | 0.980x | 1.218x |
| perft_bulk_d4 | 1.010x | 0.963x | 1.187x |
| perft_full_d3 | 0.997x | 0.986x | 1.129x |
| fen_parse | 0.914x | 0.892x | 1.082x |
| fen_write_native_ep | 1.002x | 0.991x | 1.699x |
| snapshot | 1.000x | 0.999x | 0.926x |
| san_parse_all | 0.998x | 0.945x | 2.127x |
| san_render_native | 1.001x | 0.990x | 2.320x |
| apply_children_full | 0.955x | 0.887x | 1.040x |
| make_unmake_full | 0.921x | 0.872x | 1.189x |
| startpos | 0.994x | 0.994x | 1.304x |
| chess960_materialize | 0.952x | 0.939x | 1.348x |
| chess960_perft_d3 | 1.008x | 1.005x | 1.412x |
| polyglot_key | 0.990x | 0.977x | 21.610x |
| polyglot_recompute | 1.005x | 0.995x | 0.979x |
| repetition_key | 0.946x | 0.883x | 19.236x |
| corpus_count | 1.003x | 0.999x | 1.226x |
| corpus_materialize | 0.997x | 0.996x | 1.292x |
| corpus_uci_sorted | 0.987x | 0.975x | 1.580x |
| replay_trusted_native | 0.834x | 0.799x | 0.571x |
| replay_checked_native | 0.942x | 0.924x | 0.932x |

Cold first legal move list: 0.012 ms -> 0.010 ms (1.17x).
