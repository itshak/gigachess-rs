| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.16x | 1.49x | 1.45x | 2.21x | 1.79x | 30/30 |
| materialize | 0.99x | 1.84x | 1.60x | 1.20x | 2.17x | 1.43x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.77x | 1.50x | 1.28x | 2.24x | 1.85x | 30/30 |
| perft_full_d3 | 1.03x | 1.28x | 1.41x | 1.25x | 1.84x | 1.75x | 30/30 |
| fen_parse | 1.03x | 1.42x | 1.41x | 1.12x | 2.16x | 1.26x | 29/30 |
| fen_write_native_ep | 1.00x | 4.13x | 9.41x | 1.69x | 27.34x | 2.04x | 30/30 |
| snapshot | 0.76x | 1.62x | 1.20x | 2.74x | 1.20x | 1.04x | 30/30 |
| san_parse_all | 0.96x | 2.04x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 9.75x | N/A | 2.05x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.00x | 1.15x | 3.87x | 1.26x | 1.42x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.20x | 73.58x | 130.06x | 216.19x | 140.68x | 5/5 |
| chess960_materialize | 1.01x | 1.48x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.51x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 27.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 20.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.39x | 1.79x | 1.36x | 2.55x | 2.35x | 10/10 |
| corpus_materialize | 1.01x | 1.64x | 1.58x | 1.21x | 2.08x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.49x | 3.20x | 2.43x | 3.08x | 1.80x | 10/10 |
| replay_trusted_native | 1.02x | 0.72x | 1.28x | 1.54x | 2.11x | 2.36x | 8/10 |
| replay_checked_native | 0.94x | 2.11x | 0.91x | 7.17x | 7.09x | 8.36x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
