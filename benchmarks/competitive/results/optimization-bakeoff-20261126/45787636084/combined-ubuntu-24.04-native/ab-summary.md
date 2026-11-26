| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.978x | 0.917x | 1.368x |
| materialize | 0.995x | 0.959x | 1.529x |
| perft_bulk_d4 | 0.855x | 0.779x | 1.026x |
| perft_full_d3 | 0.937x | 0.866x | 1.418x |
| fen_parse | 0.958x | 0.887x | 1.083x |
| fen_write_native_ep | 1.011x | 0.970x | 1.916x |
| snapshot | 1.419x | 1.409x | 1.150x |
| san_parse_all | 0.983x | 0.969x | 1.899x |
| san_render_native | 0.992x | 0.986x | 2.114x |
| apply_children_full | 1.025x | 1.009x | 1.099x |
| make_unmake_full | 1.013x | 0.979x | 1.146x |
| startpos | 1.094x | 1.094x | 1.118x |
| chess960_materialize | 1.053x | 1.050x | 1.372x |
| chess960_perft_d3 | 1.021x | 0.972x | 1.415x |
| polyglot_key | 0.988x | 0.980x | 38.877x |
| polyglot_recompute | 0.995x | 0.994x | 1.017x |
| repetition_key | 0.798x | 0.718x | 16.513x |
| corpus_count | 0.655x | 0.551x | 0.868x |
| corpus_materialize | 0.946x | 0.932x | 1.049x |
| corpus_uci_sorted | 1.015x | 1.012x | 1.571x |
| replay_trusted_native | 1.006x | 1.001x | 0.714x |
| replay_checked_native | 1.001x | 0.995x | 1.108x |

Cold first legal move list: 0.009 ms -> 0.006 ms (1.67x).
