| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.163x | 1.078x | 1.373x |
| materialize | 1.061x | 1.013x | 1.471x |
| perft_bulk_d4 | 1.338x | 1.262x | 1.169x |
| perft_full_d3 | 1.436x | 1.343x | 0.889x |
| fen_parse | 0.975x | 0.932x | 1.106x |
| fen_write_native_ep | 0.973x | 0.933x | 1.311x |
| snapshot | 1.236x | 1.218x | 1.151x |
| san_parse_all | 1.828x | 1.781x | 1.647x |
| san_render_native | 1.973x | 1.804x | 2.138x |
| apply_children_full | 1.460x | 1.405x | 0.990x |
| make_unmake_full | 1.224x | 1.144x | 0.927x |
| startpos | 1.422x | 1.422x | 1.053x |
| chess960_materialize | 1.152x | 1.063x | 1.348x |
| chess960_perft_d3 | 1.374x | 1.314x | 1.377x |
| polyglot_key | 1.044x | 0.841x | 36.266x |
| polyglot_recompute | 0.966x | 0.931x | 0.975x |
| repetition_key | 1.587x | 1.536x | 22.235x |
| corpus_count | 1.127x | 1.067x | 1.304x |
| corpus_materialize | 1.017x | 1.013x | 1.096x |
| corpus_uci_sorted | 1.583x | 1.571x | 0.979x |
| replay_trusted_native | 1.228x | 1.157x | 0.611x |
| replay_checked_native | 1.228x | 1.102x | 0.975x |

Cold first legal move list: 139.429 ms → 0.011 ms (12968.96x).
