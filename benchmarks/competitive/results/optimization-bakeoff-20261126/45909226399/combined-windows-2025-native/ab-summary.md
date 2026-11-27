| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.981x | 0.951x | 1.463x |
| materialize | 0.982x | 0.949x | 1.234x |
| perft_bulk_d4 | 0.974x | 0.943x | 1.344x |
| perft_full_d3 | 1.008x | 0.983x | 1.336x |
| fen_parse | 0.910x | 0.884x | 1.141x |
| fen_write_native_ep | 0.997x | 0.982x | 1.597x |
| snapshot | 1.000x | 0.987x | 0.995x |
| san_parse_all | 1.017x | 1.009x | 2.088x |
| san_render_native | 1.011x | 0.991x | 3.924x |
| apply_children_full | 0.990x | 0.943x | 1.393x |
| make_unmake_full | 0.983x | 0.965x | 1.360x |
| startpos | 0.985x | 0.985x | 1.450x |
| chess960_materialize | 0.894x | 0.811x | 1.195x |
| chess960_perft_d3 | 0.880x | 0.805x | 1.365x |
| polyglot_key | 0.964x | 0.952x | 23.946x |
| polyglot_recompute | 0.999x | 0.996x | 1.014x |
| repetition_key | 1.032x | 1.026x | 19.922x |
| corpus_count | 0.987x | 0.980x | 1.350x |
| corpus_materialize | 1.005x | 1.004x | 1.146x |
| corpus_uci_sorted | 1.006x | 1.003x | 1.426x |
| replay_trusted_native | 0.987x | 0.980x | 0.841x |
| replay_checked_native | 1.001x | 0.964x | 1.026x |

Cold first legal move list: 0.015 ms -> 0.018 ms (0.84x).
