| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.85x | 1.84x | 1.73x | 2.60x | 2.02x | 30/30 |
| materialize | 0.99x | 2.20x | 1.78x | 1.18x | 2.20x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.14x | 1.62x | 1.40x | 2.56x | 2.19x | 30/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.56x | 1.19x | 2.24x | 1.88x | 30/30 |
| fen_parse | 1.04x | 1.54x | 1.61x | 1.34x | 2.04x | 1.48x | 30/30 |
| fen_write_native_ep | 0.98x | 4.16x | 9.42x | 1.82x | 26.83x | 2.34x | 30/30 |
| snapshot | 1.00x | 1.90x | 1.25x | 3.77x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.28x | N/A | 3.98x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 9.45x | N/A | 1.99x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.01x | 1.33x | 4.70x | 1.52x | 1.80x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.05x | 1.20x | 98.68x | 149.36x | 215.12x | 160.28x | 5/5 |
| chess960_materialize | 1.01x | 2.11x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.13x | 1.64x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.39x | 1.77x | 1.29x | 2.38x | 2.05x | 9/10 |
| corpus_materialize | 1.00x | 1.92x | 1.79x | 1.23x | 2.83x | 1.56x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.34x | 3.19x | 2.34x | 3.11x | 1.66x | 10/10 |
| replay_trusted_native | 0.96x | 0.97x | 1.69x | 1.96x | 3.24x | 3.96x | 9/10 |
| replay_checked_native | 0.96x | 2.81x | 1.05x | 6.06x | 6.77x | 8.21x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
