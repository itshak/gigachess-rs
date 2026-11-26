| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.76x | 1.61x | 1.68x | 2.30x | 1.94x | 28/30 |
| materialize | 0.99x | 1.88x | 1.54x | 1.11x | 2.28x | 1.28x | 27/30 |
| perft_bulk_d4 | 0.99x | 3.60x | 1.50x | 1.35x | 1.92x | 1.83x | 30/30 |
| perft_full_d3 | 1.16x | 1.69x | 1.90x | 1.49x | 2.10x | 2.26x | 30/30 |
| fen_parse | 1.04x | 1.25x | 1.52x | 1.38x | 1.62x | 1.57x | 30/30 |
| fen_write_native_ep | 0.94x | 4.12x | 7.77x | 1.51x | 27.32x | 2.18x | 30/30 |
| snapshot | 0.64x | 1.22x | 0.85x | 1.61x | 0.97x | 0.75x | 15/30 |
| san_parse_all | 1.16x | 1.96x | N/A | 4.03x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.57x | N/A | 3.03x | N/A | N/A | 12/12 |
| apply_children_full | 1.12x | 1.27x | 1.35x | 6.08x | 1.18x | 1.44x | 29/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 0.50x | 1.24x | 33.65x | 67.22x | 77.23x | 86.30x | 5/5 |
| chess960_materialize | 0.87x | 1.83x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 3.48x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 34.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.87x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 20.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.86x | 5.84x | 1.76x | 1.15x | 2.15x | 2.15x | 9/10 |
| corpus_materialize | 1.17x | 1.66x | 1.55x | 1.32x | 1.93x | 1.43x | 10/10 |
| corpus_uci_sorted | 0.93x | 1.72x | 2.38x | 1.63x | 2.24x | 1.41x | 10/10 |
| replay_trusted_native | 1.08x | 0.65x | 1.79x | 1.81x | 1.81x | 1.98x | 8/10 |
| replay_checked_native | 0.91x | 2.01x | 1.15x | 9.40x | 7.56x | 10.36x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
