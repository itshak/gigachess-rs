| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.14x | 1.61x | 1.45x | 2.07x | 1.44x | 30/30 |
| materialize | 0.99x | 2.10x | 1.87x | 1.34x | 2.51x | 1.62x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.06x | 1.68x | 1.30x | 2.10x | 1.81x | 30/30 |
| perft_full_d3 | 1.19x | 1.62x | 1.89x | 1.54x | 2.31x | 2.11x | 30/30 |
| fen_parse | 1.01x | 1.30x | 1.40x | 1.09x | 2.02x | 1.21x | 30/30 |
| fen_write_native_ep | 1.04x | 2.95x | 9.65x | 1.72x | 28.64x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.22x | 3.07x | 1.02x | 0.97x | 24/30 |
| san_parse_all | 1.00x | 2.03x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.64x | N/A | 2.33x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.20x | 1.55x | 4.55x | 1.44x | 1.48x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.42x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 65.19x | 108.26x | 179.81x | 114.92x | 5/5 |
| chess960_materialize | 0.97x | 1.62x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.33x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 23.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 17.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 3.89x | 1.97x | 1.30x | 2.14x | 2.58x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.62x | 1.21x | 2.11x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.24x | 3.23x | 2.46x | 3.11x | 1.83x | 10/10 |
| replay_trusted_native | 0.99x | 0.89x | 1.72x | 1.97x | 2.69x | 2.47x | 9/10 |
| replay_checked_native | 1.00x | 2.34x | 1.13x | 8.12x | 8.20x | 9.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
