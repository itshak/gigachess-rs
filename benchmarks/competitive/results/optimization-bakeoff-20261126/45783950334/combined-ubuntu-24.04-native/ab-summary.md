| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.991x | 1.326x |
| materialize | 0.970x | 0.951x | 1.149x |
| perft_bulk_d4 | 0.984x | 0.955x | 1.246x |
| perft_full_d3 | 0.955x | 0.936x | 0.956x |
| fen_parse | 1.065x | 1.053x | 1.252x |
| fen_write_native_ep | 0.996x | 0.988x | 1.910x |
| snapshot | 0.655x | 0.595x | 0.973x |
| san_parse_all | 1.004x | 0.997x | 2.254x |
| san_render_native | 1.011x | 0.966x | 2.159x |
| apply_children_full | 1.018x | 0.931x | 1.020x |
| make_unmake_full | 0.954x | 0.905x | 1.149x |
| startpos | 0.834x | 0.834x | 1.168x |
| chess960_materialize | 1.045x | 1.013x | 1.420x |
| chess960_perft_d3 | 0.990x | 0.990x | 1.408x |
| polyglot_key | 0.998x | 0.996x | 20.714x |
| polyglot_recompute | 0.958x | 0.942x | 0.997x |
| repetition_key | 0.937x | 0.884x | 18.220x |
| corpus_count | 1.007x | 1.005x | 1.322x |
| corpus_materialize | 0.981x | 0.972x | 1.215x |
| corpus_uci_sorted | 0.985x | 0.982x | 1.685x |
| replay_trusted_native | 0.876x | 0.850x | 0.651x |
| replay_checked_native | 0.969x | 0.938x | 0.911x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.93x).
