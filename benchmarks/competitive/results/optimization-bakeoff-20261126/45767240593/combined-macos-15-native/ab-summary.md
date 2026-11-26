| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.217x | 1.022x | 1.253x |
| materialize | 1.002x | 0.844x | 0.947x |
| perft_bulk_d4 | 1.151x | 0.916x | 1.113x |
| perft_full_d3 | 1.335x | 1.091x | 0.746x |
| fen_parse | 1.100x | 0.998x | 1.245x |
| fen_write_native_ep | 1.010x | 0.915x | 1.349x |
| snapshot | 1.271x | 1.071x | 0.940x |
| san_parse_all | 1.508x | 1.420x | 1.682x |
| san_render_native | 1.808x | 1.534x | 2.739x |
| apply_children_full | 1.234x | 1.083x | 0.777x |
| make_unmake_full | 1.252x | 1.134x | 1.028x |
| startpos | 1.452x | 1.452x | 1.188x |
| chess960_materialize | 1.021x | 1.009x | 1.085x |
| chess960_perft_d3 | 1.306x | 1.304x | 1.293x |
| polyglot_key | 1.072x | 0.964x | 39.599x |
| polyglot_recompute | 1.093x | 0.913x | 1.050x |
| repetition_key | 1.195x | 1.178x | 18.766x |
| corpus_count | 1.344x | 1.161x | 1.244x |
| corpus_materialize | 1.117x | 1.114x | 1.109x |
| corpus_uci_sorted | 1.306x | 1.274x | 0.761x |
| replay_trusted_native | 1.272x | 1.123x | 0.443x |
| replay_checked_native | 1.199x | 1.146x | 0.893x |

Cold first legal move list: 138.797 ms → 0.008 ms (16410.17x).
