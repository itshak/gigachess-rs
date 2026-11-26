| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.934x | 1.315x |
| materialize | 1.104x | 1.030x | 1.148x |
| perft_bulk_d4 | 0.966x | 0.906x | 1.155x |
| perft_full_d3 | 1.101x | 0.890x | 1.023x |
| fen_parse | 1.146x | 1.119x | 1.098x |
| fen_write_native_ep | 1.014x | 0.995x | 1.467x |
| snapshot | 1.159x | 1.108x | 1.491x |
| san_parse_all | 0.991x | 0.952x | 1.741x |
| san_render_native | 1.053x | 1.013x | 2.337x |
| apply_children_full | 1.220x | 1.157x | 1.485x |
| make_unmake_full | 1.040x | 0.989x | 1.306x |
| startpos | 0.975x | 0.975x | 1.251x |
| chess960_materialize | 1.044x | 0.981x | 1.385x |
| chess960_perft_d3 | 1.011x | 0.953x | 1.384x |
| polyglot_key | 0.996x | 0.975x | 35.805x |
| polyglot_recompute | 0.986x | 0.982x | 0.950x |
| repetition_key | 0.842x | 0.803x | 22.421x |
| corpus_count | 0.885x | 0.789x | 1.081x |
| corpus_materialize | 0.886x | 0.816x | 1.036x |
| corpus_uci_sorted | 0.981x | 0.968x | 1.638x |
| replay_trusted_native | 0.968x | 0.940x | 0.644x |
| replay_checked_native | 1.012x | 1.007x | 0.915x |

Cold first legal move list: 0.012 ms -> 0.005 ms (2.16x).
