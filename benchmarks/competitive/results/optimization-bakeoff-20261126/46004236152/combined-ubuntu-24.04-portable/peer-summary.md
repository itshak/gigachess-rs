| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.05x | 1.74x | 1.51x | 2.03x | 1.45x | 30/30 |
| materialize | 1.00x | 2.30x | 1.87x | 1.92x | 2.81x | 2.03x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.21x | 1.70x | 1.27x | 2.12x | 1.90x | 29/30 |
| perft_full_d3 | 1.16x | 1.59x | 1.91x | 1.63x | 2.28x | 1.87x | 30/30 |
| fen_parse | 1.00x | 1.46x | 1.40x | 1.18x | 1.47x | 1.17x | 30/30 |
| fen_write_native_ep | 1.02x | 3.24x | 10.20x | 2.07x | 28.07x | 2.52x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.01x | 1.94x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.24x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.24x | 1.50x | 4.32x | 1.39x | 1.59x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 61.34x | 109.81x | 119.56x | 96.14x | 5/5 |
| chess960_materialize | 1.06x | 1.81x | 1.77x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.15x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 40.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.20x | 24.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 4.90x | 2.44x | 1.34x | 2.48x | 3.35x | 10/10 |
| corpus_materialize | 0.99x | 1.50x | 1.41x | 1.13x | 1.84x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.97x | 3.08x | 2.23x | 2.94x | 1.55x | 10/10 |
| replay_trusted_native | 1.03x | 0.90x | 1.72x | 1.71x | 2.28x | 2.10x | 9/10 |
| replay_checked_native | 1.01x | 2.07x | 1.18x | 9.24x | 8.74x | 10.00x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
