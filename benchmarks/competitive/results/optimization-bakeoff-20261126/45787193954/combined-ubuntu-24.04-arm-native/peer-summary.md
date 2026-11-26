| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.85x | 1.46x | 1.58x | 1.92x | 1.90x | 30/30 |
| materialize | 1.00x | 1.82x | 1.42x | 1.26x | 2.02x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.74x | 1.52x | 1.38x | 1.95x | 2.01x | 30/30 |
| perft_full_d3 | 1.03x | 1.20x | 1.31x | 1.08x | 1.60x | 1.51x | 29/30 |
| fen_parse | 1.01x | 1.26x | 1.35x | 1.30x | 1.77x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 4.51x | 9.41x | 1.57x | 30.09x | 2.40x | 30/30 |
| snapshot | 0.92x | 1.05x | 1.08x | 1.44x | 1.08x | 0.95x | 24/30 |
| san_parse_all | 0.99x | 1.82x | N/A | 4.13x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.79x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 1.24x | 1.48x | 4.01x | 1.19x | 1.19x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.42x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.33x | 76.68x | 129.77x | 168.27x | 136.24x | 5/5 |
| chess960_materialize | 1.00x | 1.52x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.64x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 18.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.74x | 1.84x | 1.47x | 2.14x | 2.35x | 10/10 |
| corpus_materialize | 1.01x | 1.57x | 1.51x | 1.21x | 1.87x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.91x | 2.12x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.50x | 1.33x | 1.59x | 1.78x | 1.81x | 8/10 |
| replay_checked_native | 1.01x | 2.02x | 0.99x | 6.71x | 5.92x | 7.66x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
