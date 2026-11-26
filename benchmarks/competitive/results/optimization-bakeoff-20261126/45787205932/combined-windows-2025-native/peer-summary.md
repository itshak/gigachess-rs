| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.54x | 1.48x | 1.39x | 2.16x | 1.82x | 30/30 |
| materialize | 1.00x | 2.07x | 1.36x | 1.17x | 2.34x | 1.46x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.01x | 1.53x | 1.26x | 2.37x | 2.18x | 30/30 |
| perft_full_d3 | 0.68x | 0.92x | 1.08x | 0.84x | 1.43x | 1.59x | 20/30 |
| fen_parse | 1.09x | 1.37x | 1.50x | 1.51x | 2.14x | 1.56x | 30/30 |
| fen_write_native_ep | 0.99x | 3.59x | 7.02x | 1.50x | 31.44x | 2.05x | 30/30 |
| snapshot | 1.13x | 2.01x | 1.24x | 3.57x | 1.19x | 1.19x | 30/30 |
| san_parse_all | 1.04x | 1.99x | N/A | 4.77x | N/A | N/A | 12/12 |
| san_render_native | 1.10x | 10.77x | N/A | 3.82x | N/A | N/A | 12/12 |
| apply_children_full | 1.22x | 1.63x | 1.27x | 6.68x | 1.06x | 2.11x | 29/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.15x | 1.25x | 94.32x | 169.71x | 238.72x | 180.80x | 5/5 |
| chess960_materialize | 1.01x | 1.43x | 1.16x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.63x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.92x | 33.60x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.87x | 14.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 6.26x | 1.84x | 1.22x | 2.39x | 2.35x | 10/10 |
| corpus_materialize | 1.14x | 1.82x | 1.43x | 1.19x | 2.45x | 1.61x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.72x | 1.99x | 1.74x | 1.98x | 1.38x | 10/10 |
| replay_trusted_native | 0.98x | 0.62x | 1.18x | 1.39x | 2.08x | 2.66x | 8/10 |
| replay_checked_native | 1.02x | 2.00x | 0.94x | 7.10x | 7.23x | 8.92x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
