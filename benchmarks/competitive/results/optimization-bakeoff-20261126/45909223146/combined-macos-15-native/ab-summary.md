| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.851x | 1.542x |
| materialize | 1.057x | 0.972x | 1.128x |
| perft_bulk_d4 | 1.044x | 0.842x | 1.316x |
| perft_full_d3 | 1.065x | 0.705x | 1.434x |
| fen_parse | 0.983x | 0.823x | 1.133x |
| fen_write_native_ep | 0.910x | 0.643x | 1.571x |
| snapshot | 1.121x | 0.583x | 0.916x |
| san_parse_all | 1.011x | 0.813x | 1.764x |
| san_render_native | 1.085x | 0.783x | 2.772x |
| apply_children_full | 1.101x | 0.905x | 1.107x |
| make_unmake_full | 1.004x | 0.795x | 1.324x |
| startpos | 0.896x | 0.896x | 1.757x |
| chess960_materialize | 0.932x | 0.913x | 1.907x |
| chess960_perft_d3 | 0.721x | 0.645x | 1.594x |
| polyglot_key | 0.683x | 0.623x | 37.440x |
| polyglot_recompute | 1.014x | 0.931x | 1.150x |
| repetition_key | 0.965x | 0.863x | 22.177x |
| corpus_count | 0.971x | 0.838x | 1.659x |
| corpus_materialize | 0.757x | 0.739x | 0.972x |
| corpus_uci_sorted | 0.730x | 0.601x | 1.332x |
| replay_trusted_native | 0.696x | 0.667x | 0.544x |
| replay_checked_native | 0.807x | 0.749x | 1.079x |

Cold first legal move list: 0.003 ms -> 0.005 ms (0.71x).
