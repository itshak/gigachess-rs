| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.08x | 1.44x | 1.39x | 2.03x | 1.79x | 30/30 |
| materialize | 0.98x | 1.83x | 1.25x | 1.14x | 2.14x | 1.45x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.60x | 1.41x | 1.23x | 2.04x | 1.81x | 30/30 |
| perft_full_d3 | 1.00x | 1.33x | 1.41x | 1.26x | 1.87x | 1.79x | 30/30 |
| fen_parse | 1.07x | 1.34x | 1.48x | 1.36x | 2.30x | 1.66x | 30/30 |
| fen_write_native_ep | 1.00x | 4.15x | 8.22x | 1.57x | 31.28x | 1.96x | 30/30 |
| snapshot | 0.99x | 1.83x | 1.00x | 4.07x | 1.05x | 1.01x | 25/30 |
| san_parse_all | 1.03x | 2.09x | N/A | 5.62x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.62x | N/A | 3.73x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.22x | 1.35x | 9.21x | 1.41x | 1.59x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.26x | 79.85x | 152.33x | 230.31x | 182.81x | 5/5 |
| chess960_materialize | 1.01x | 1.54x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.68x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 30.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.05x | 21.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.84x | 1.67x | 1.22x | 2.24x | 2.16x | 10/10 |
| corpus_materialize | 1.01x | 1.50x | 1.26x | 1.08x | 1.87x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.79x | 2.19x | 1.81x | 2.13x | 1.43x | 10/10 |
| replay_trusted_native | 0.99x | 0.83x | 1.57x | 2.04x | 2.56x | 2.97x | 9/10 |
| replay_checked_native | 1.01x | 2.28x | 1.00x | 7.57x | 7.58x | 9.05x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
