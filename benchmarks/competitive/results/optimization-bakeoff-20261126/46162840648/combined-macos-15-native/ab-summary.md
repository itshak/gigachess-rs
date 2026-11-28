| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.160x | 0.991x | 1.640x |
| materialize | 1.195x | 1.043x | 1.137x |
| perft_bulk_d4 | 1.184x | 1.047x | 1.328x |
| perft_full_d3 | 1.367x | 1.169x | 1.480x |
| fen_parse | 1.072x | 0.963x | 1.263x |
| fen_write_native_ep | 1.189x | 0.912x | 1.770x |
| snapshot | 0.981x | 0.639x | 0.899x |
| san_parse_all | 1.058x | 0.921x | 1.805x |
| san_render_native | 1.120x | 0.983x | 2.907x |
| apply_children_full | 1.159x | 1.017x | 1.174x |
| make_unmake_full | 1.119x | 0.949x | 1.300x |
| startpos | 1.012x | 1.012x | 1.200x |
| chess960_materialize | 1.073x | 0.969x | 1.628x |
| chess960_perft_d3 | 1.003x | 0.874x | 1.378x |
| polyglot_key | 1.090x | 1.071x | 36.664x |
| polyglot_recompute | 1.622x | 1.616x | 1.544x |
| repetition_key | 0.975x | 0.840x | 18.935x |
| corpus_count | 1.089x | 1.012x | 1.125x |
| corpus_materialize | 0.975x | 0.968x | 1.069x |
| corpus_uci_sorted | 0.890x | 0.833x | 1.452x |
| replay_trusted_native | 0.908x | 0.879x | 0.753x |
| replay_checked_native | 1.126x | 0.904x | 1.215x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.11x).
