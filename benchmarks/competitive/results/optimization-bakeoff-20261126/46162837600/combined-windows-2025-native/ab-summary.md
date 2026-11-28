| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.145x | 1.030x | 1.584x |
| materialize | 1.039x | 0.973x | 1.280x |
| perft_bulk_d4 | 1.029x | 0.993x | 1.398x |
| perft_full_d3 | 1.304x | 1.053x | 1.609x |
| fen_parse | 0.827x | 0.760x | 1.093x |
| fen_write_native_ep | 1.042x | 0.994x | 1.713x |
| snapshot | 1.003x | 0.993x | 0.998x |
| san_parse_all | 1.017x | 1.000x | 2.098x |
| san_render_native | 1.031x | 0.995x | 3.983x |
| apply_children_full | 1.000x | 0.921x | 1.391x |
| make_unmake_full | 1.029x | 0.986x | 1.370x |
| startpos | 1.003x | 1.003x | 1.270x |
| chess960_materialize | 0.950x | 0.947x | 1.292x |
| chess960_perft_d3 | 0.997x | 0.981x | 1.580x |
| polyglot_key | 0.868x | 0.788x | 23.605x |
| polyglot_recompute | 1.271x | 1.186x | 1.297x |
| repetition_key | 0.924x | 0.879x | 19.393x |
| corpus_count | 1.086x | 1.064x | 1.484x |
| corpus_materialize | 1.010x | 1.002x | 1.180x |
| corpus_uci_sorted | 0.988x | 0.986x | 1.393x |
| replay_trusted_native | 1.037x | 1.005x | 0.877x |
| replay_checked_native | 0.982x | 0.978x | 1.058x |

Cold first legal move list: 0.019 ms -> 0.016 ms (1.17x).
