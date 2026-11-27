| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.10x | 5.20x | 1.73x | 1.93x | 2.56x | 2.30x | 30/30 |
| materialize | 1.08x | 2.04x | 1.58x | 1.06x | 2.18x | 1.32x | 28/30 |
| perft_bulk_d4 | 1.17x | 4.24x | 1.62x | 1.33x | 2.07x | 2.00x | 30/30 |
| perft_full_d3 | 1.13x | 1.55x | 1.66x | 1.29x | 1.92x | 2.01x | 30/30 |
| fen_parse | 1.06x | 1.25x | 1.26x | 1.30x | 1.56x | 1.62x | 30/30 |
| fen_write_native_ep | 1.01x | 4.17x | 8.40x | 1.85x | 25.84x | 2.12x | 30/30 |
| snapshot | 1.20x | 1.78x | 1.15x | 2.48x | 1.13x | 1.01x | 28/30 |
| san_parse_all | 1.10x | 1.71x | N/A | 3.56x | N/A | N/A | 12/12 |
| san_render_native | 1.10x | 12.64x | N/A | 2.91x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.16x | 1.21x | 5.62x | 1.16x | 1.28x | 30/30 |
| make_unmake_full | 1.10x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.22x | 69.32x | 159.91x | 159.03x | 177.31x | 5/5 |
| chess960_materialize | 1.03x | 1.77x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.20x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.14x | 38.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.23x | 20.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 5.96x | 2.00x | 1.21x | 2.27x | 2.66x | 10/10 |
| corpus_materialize | 1.13x | 1.52x | 1.43x | 1.11x | 1.89x | 1.18x | 9/10 |
| corpus_uci_sorted | 1.13x | 1.88x | 2.55x | 1.86x | 2.43x | 1.35x | 10/10 |
| replay_trusted_native | 0.97x | 0.65x | 1.60x | 1.71x | 1.73x | 1.86x | 8/10 |
| replay_checked_native | 1.08x | 2.63x | 1.26x | 9.21x | 8.18x | 9.98x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
