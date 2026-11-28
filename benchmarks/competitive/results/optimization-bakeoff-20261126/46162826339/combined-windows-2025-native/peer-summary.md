| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 5.08x | 1.78x | 1.80x | 2.50x | 2.24x | 30/30 |
| materialize | 1.04x | 2.04x | 1.40x | 1.40x | 2.44x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.13x | 1.70x | 1.43x | 2.42x | 2.09x | 30/30 |
| perft_full_d3 | 1.26x | 1.71x | 1.83x | 1.66x | 2.37x | 2.30x | 30/30 |
| fen_parse | 0.99x | 1.33x | 1.55x | 1.50x | 2.32x | 1.49x | 30/30 |
| fen_write_native_ep | 1.01x | 4.13x | 8.13x | 1.55x | 31.02x | 2.04x | 30/30 |
| snapshot | 0.98x | 1.97x | 1.03x | 4.02x | 1.04x | 1.03x | 29/30 |
| san_parse_all | 0.97x | 2.08x | N/A | 5.51x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.20x | N/A | 4.01x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.40x | 1.43x | 10.51x | 1.62x | 1.70x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.25x | 78.84x | 175.50x | 259.67x | 179.59x | 5/5 |
| chess960_materialize | 0.98x | 1.61x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.08x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 29.95x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.17x | 1.27x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.87x | 20.52x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 5.73x | 1.94x | 1.53x | 2.72x | 2.65x | 10/10 |
| corpus_materialize | 0.98x | 1.59x | 1.24x | 1.21x | 2.09x | 1.43x | 10/10 |
| corpus_uci_sorted | 0.93x | 1.76x | 2.17x | 1.74x | 2.05x | 1.36x | 10/10 |
| replay_trusted_native | 1.02x | 0.92x | 1.76x | 2.50x | 2.72x | 3.20x | 9/10 |
| replay_checked_native | 0.80x | 2.15x | 0.92x | 7.35x | 7.71x | 8.03x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
