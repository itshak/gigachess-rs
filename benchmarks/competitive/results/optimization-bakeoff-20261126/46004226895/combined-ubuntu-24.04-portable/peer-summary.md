| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.02x | 1.74x | 1.51x | 2.02x | 1.49x | 30/30 |
| materialize | 1.00x | 2.28x | 1.95x | 1.91x | 2.90x | 2.04x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.23x | 1.70x | 1.27x | 2.12x | 1.94x | 30/30 |
| perft_full_d3 | 1.17x | 1.61x | 1.86x | 1.63x | 2.21x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.58x | 1.48x | 1.24x | 1.56x | 1.27x | 30/30 |
| fen_write_native_ep | 1.01x | 3.07x | 9.99x | 1.77x | 27.58x | 2.56x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 0.92x | 1.78x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.29x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.22x | 1.46x | 4.17x | 1.36x | 1.58x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 56.94x | 112.72x | 126.16x | 109.18x | 5/5 |
| chess960_materialize | 0.94x | 1.70x | 1.66x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.14x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.23x | 24.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 4.88x | 2.44x | 1.34x | 2.48x | 3.42x | 10/10 |
| corpus_materialize | 0.99x | 1.50x | 1.40x | 1.13x | 1.84x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.95x | 2.94x | 2.21x | 2.87x | 1.54x | 10/10 |
| replay_trusted_native | 0.99x | 0.91x | 1.61x | 1.62x | 2.23x | 2.05x | 9/10 |
| replay_checked_native | 1.01x | 2.11x | 1.13x | 9.00x | 8.77x | 9.99x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
