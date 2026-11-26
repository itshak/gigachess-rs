| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.026x | 0.988x | 1.506x |
| materialize | 0.994x | 0.960x | 1.484x |
| perft_bulk_d4 | 1.052x | 1.008x | 1.230x |
| perft_full_d3 | 0.998x | 0.993x | 1.349x |
| fen_parse | 0.978x | 0.968x | 1.147x |
| fen_write_native_ep | 1.002x | 0.992x | 1.924x |
| snapshot | 1.002x | 0.980x | 1.004x |
| san_parse_all | 0.925x | 0.889x | 1.773x |
| san_render_native | 0.984x | 0.978x | 2.096x |
| apply_children_full | 0.998x | 0.991x | 1.358x |
| make_unmake_full | 0.992x | 0.975x | 1.182x |
| startpos | 0.896x | 0.896x | 0.999x |
| chess960_materialize | 1.028x | 0.996x | 1.420x |
| chess960_perft_d3 | 0.997x | 0.996x | 1.498x |
| polyglot_key | 1.003x | 0.999x | 40.430x |
| polyglot_recompute | 0.960x | 0.947x | 0.965x |
| repetition_key | 1.051x | 0.851x | 27.056x |
| corpus_count | 1.387x | 1.113x | 1.283x |
| corpus_materialize | 1.019x | 1.017x | 1.063x |
| corpus_uci_sorted | 0.994x | 0.991x | 1.552x |
| replay_trusted_native | 0.988x | 0.968x | 0.836x |
| replay_checked_native | 1.029x | 1.027x | 1.271x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.98x).
