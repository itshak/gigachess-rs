| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.12x | 1.62x | 1.42x | 1.96x | 1.43x | 30/30 |
| materialize | 1.01x | 2.32x | 1.83x | 1.32x | 2.60x | 1.66x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.00x | 1.55x | 1.18x | 1.94x | 1.78x | 30/30 |
| perft_full_d3 | 1.05x | 1.14x | 1.41x | 1.09x | 1.58x | 1.45x | 28/30 |
| fen_parse | 1.01x | 1.55x | 1.50x | 1.21x | 1.91x | 1.23x | 30/30 |
| fen_write_native_ep | 1.03x | 2.72x | 8.57x | 1.54x | 25.47x | 2.04x | 30/30 |
| snapshot | 1.00x | 1.62x | 1.16x | 2.62x | 1.15x | 0.99x | 24/30 |
| san_parse_all | 0.99x | 1.79x | N/A | 3.82x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.43x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 2.18x | 1.48x | 4.85x | 1.26x | 1.31x | 28/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.28x | 58.98x | 82.48x | 114.07x | 79.46x | 5/5 |
| chess960_materialize | 1.00x | 1.79x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.30x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 32.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.18x | 24.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.26x | 1.78x | 1.13x | 2.01x | 2.88x | 9/10 |
| corpus_materialize | 0.97x | 1.76x | 1.64x | 1.17x | 2.31x | 1.43x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.02x | 2.78x | 2.30x | 2.83x | 1.59x | 10/10 |
| replay_trusted_native | 1.11x | 0.78x | 1.75x | 1.75x | 2.39x | 2.14x | 9/10 |
| replay_checked_native | 1.08x | 2.32x | 1.29x | 9.08x | 8.77x | 10.38x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
