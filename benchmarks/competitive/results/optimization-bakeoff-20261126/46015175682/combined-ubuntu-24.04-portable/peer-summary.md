| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.31x | 1.72x | 1.56x | 2.18x | 1.56x | 30/30 |
| materialize | 1.03x | 2.38x | 2.05x | 1.37x | 2.70x | 1.73x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.21x | 1.71x | 1.37x | 2.14x | 1.85x | 30/30 |
| perft_full_d3 | 1.24x | 1.74x | 2.07x | 1.60x | 2.43x | 2.27x | 30/30 |
| fen_parse | 1.02x | 1.54x | 1.56x | 1.20x | 2.38x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 2.59x | 9.08x | 1.68x | 25.49x | 2.06x | 30/30 |
| snapshot | 1.00x | 1.63x | 1.22x | 2.82x | 1.14x | 0.99x | 25/30 |
| san_parse_all | 1.03x | 2.26x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.91x | N/A | 2.30x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.42x | 1.53x | 5.79x | 1.44x | 1.51x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 72.90x | 109.28x | 181.27x | 117.52x | 5/5 |
| chess960_materialize | 0.99x | 1.92x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.44x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.22x | 1.19x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 20.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 3.43x | 1.91x | 1.33x | 2.07x | 1.99x | 10/10 |
| corpus_materialize | 1.00x | 2.06x | 1.84x | 1.35x | 3.04x | 1.74x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 3.01x | 2.25x | 2.84x | 1.61x | 10/10 |
| replay_trusted_native | 1.01x | 1.08x | 2.21x | 2.37x | 3.52x | 3.22x | 10/10 |
| replay_checked_native | 1.00x | 2.55x | 1.16x | 6.26x | 7.43x | 7.97x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
