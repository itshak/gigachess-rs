| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.957x | 0.677x | 1.518x |
| materialize | 1.042x | 0.864x | 1.197x |
| perft_bulk_d4 | 0.938x | 0.672x | 1.222x |
| perft_full_d3 | 0.992x | 0.853x | 1.361x |
| fen_parse | 0.893x | 0.747x | 1.167x |
| fen_write_native_ep | 0.882x | 0.794x | 1.713x |
| snapshot | 0.928x | 0.666x | 0.811x |
| san_parse_all | 0.941x | 0.816x | 1.752x |
| san_render_native | 0.964x | 0.803x | 2.624x |
| apply_children_full | 0.968x | 0.892x | 1.099x |
| make_unmake_full | 1.009x | 0.712x | 1.277x |
| startpos | 0.605x | 0.605x | 0.743x |
| chess960_materialize | 0.982x | 0.966x | 1.658x |
| chess960_perft_d3 | 0.951x | 0.907x | 1.396x |
| polyglot_key | 0.955x | 0.930x | 39.647x |
| polyglot_recompute | 0.949x | 0.904x | 0.941x |
| repetition_key | 0.927x | 0.917x | 18.129x |
| corpus_count | 1.034x | 1.010x | 1.055x |
| corpus_materialize | 0.843x | 0.767x | 1.177x |
| corpus_uci_sorted | 1.039x | 1.000x | 1.571x |
| replay_trusted_native | 0.964x | 0.856x | 0.618x |
| replay_checked_native | 0.975x | 0.852x | 1.218x |

Cold first legal move list: 0.006 ms -> 0.008 ms (0.79x).
