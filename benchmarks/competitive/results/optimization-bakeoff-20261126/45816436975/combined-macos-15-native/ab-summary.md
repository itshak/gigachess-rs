| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.014x | 0.911x | 1.333x |
| materialize | 1.121x | 0.992x | 1.051x |
| perft_bulk_d4 | 0.820x | 0.667x | 1.205x |
| perft_full_d3 | 0.968x | 0.778x | 1.445x |
| fen_parse | 0.979x | 0.868x | 1.194x |
| fen_write_native_ep | 0.911x | 0.745x | 1.576x |
| snapshot | 1.045x | 0.826x | 0.969x |
| san_parse_all | 0.909x | 0.814x | 1.725x |
| san_render_native | 1.113x | 0.908x | 2.927x |
| apply_children_full | 1.142x | 0.984x | 1.088x |
| make_unmake_full | 1.119x | 0.926x | 1.230x |
| startpos | 0.774x | 0.774x | 0.981x |
| chess960_materialize | 0.979x | 0.885x | 1.418x |
| chess960_perft_d3 | 0.826x | 0.698x | 1.262x |
| polyglot_key | 0.993x | 0.990x | 40.781x |
| polyglot_recompute | 0.967x | 0.899x | 1.020x |
| repetition_key | 1.213x | 1.117x | 18.811x |
| corpus_count | 0.823x | 0.800x | 1.132x |
| corpus_materialize | 0.967x | 0.948x | 1.256x |
| corpus_uci_sorted | 0.971x | 0.947x | 1.290x |
| replay_trusted_native | 1.150x | 0.769x | 0.578x |
| replay_checked_native | 0.966x | 0.894x | 0.995x |

Cold first legal move list: 0.014 ms -> 0.006 ms (2.17x).
