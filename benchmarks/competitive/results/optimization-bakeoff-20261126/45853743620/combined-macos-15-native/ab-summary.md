| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.138x | 0.943x | 1.541x |
| materialize | 1.094x | 0.952x | 1.073x |
| perft_bulk_d4 | 1.155x | 1.065x | 1.250x |
| perft_full_d3 | 1.035x | 0.805x | 1.234x |
| fen_parse | 1.043x | 0.934x | 1.213x |
| fen_write_native_ep | 1.086x | 0.958x | 1.800x |
| snapshot | 1.017x | 0.882x | 1.002x |
| san_parse_all | 1.080x | 0.960x | 1.681x |
| san_render_native | 1.094x | 0.954x | 2.563x |
| apply_children_full | 1.266x | 1.171x | 1.218x |
| make_unmake_full | 1.167x | 1.046x | 1.383x |
| startpos | 0.983x | 0.983x | 1.269x |
| chess960_materialize | 1.077x | 1.046x | 1.523x |
| chess960_perft_d3 | 1.129x | 1.085x | 1.571x |
| polyglot_key | 1.104x | 1.024x | 39.688x |
| polyglot_recompute | 1.159x | 1.158x | 1.070x |
| repetition_key | 1.033x | 1.010x | 20.837x |
| corpus_count | 0.980x | 0.975x | 1.051x |
| corpus_materialize | 1.102x | 1.080x | 1.067x |
| corpus_uci_sorted | 1.099x | 1.020x | 1.393x |
| replay_trusted_native | 1.269x | 1.118x | 0.765x |
| replay_checked_native | 0.981x | 0.889x | 1.129x |

Cold first legal move list: 0.004 ms -> 0.002 ms (1.71x).
