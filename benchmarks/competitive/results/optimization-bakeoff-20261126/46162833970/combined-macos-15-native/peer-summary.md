| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.78x | 4.80x | 1.61x | 1.54x | 1.93x | 1.87x | 30/30 |
| materialize | 0.86x | 1.91x | 1.46x | 1.18x | 2.15x | 1.56x | 30/30 |
| perft_bulk_d4 | 0.86x | 3.99x | 1.42x | 1.29x | 1.86x | 1.97x | 29/30 |
| perft_full_d3 | 0.94x | 1.64x | 1.76x | 1.36x | 2.03x | 1.95x | 30/30 |
| fen_parse | 0.91x | 1.38x | 1.56x | 1.49x | 1.80x | 1.52x | 30/30 |
| fen_write_native_ep | 0.85x | 4.07x | 8.60x | 1.61x | 25.92x | 2.00x | 30/30 |
| snapshot | 0.79x | 1.47x | 0.90x | 2.06x | 1.01x | 0.86x | 19/30 |
| san_parse_all | 0.99x | 1.96x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 12.84x | N/A | 3.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 1.18x | 1.27x | 6.23x | 1.11x | 1.35x | 27/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.40x | N/A | N/A | 6/6 |
| startpos | 0.94x | 0.77x | 61.12x | 119.52x | 144.55x | 129.54x | 4/5 |
| chess960_materialize | 1.05x | 2.03x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.10x | 3.61x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 42.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.58x | 1.63x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.83x | 21.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 6.21x | 2.39x | 1.20x | 2.47x | 3.05x | 10/10 |
| corpus_materialize | 0.92x | 1.67x | 1.64x | 1.32x | 1.93x | 1.59x | 10/10 |
| corpus_uci_sorted | 0.85x | 1.80x | 2.36x | 1.76x | 2.44x | 1.24x | 10/10 |
| replay_trusted_native | 0.82x | 0.63x | 1.58x | 1.62x | 1.84x | 1.65x | 8/10 |
| replay_checked_native | 0.75x | 3.04x | 1.37x | 9.13x | 8.92x | 10.35x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
