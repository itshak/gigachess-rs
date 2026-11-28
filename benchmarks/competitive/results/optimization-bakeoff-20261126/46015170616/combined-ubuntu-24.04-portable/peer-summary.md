| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 3.06x | 1.75x | 1.52x | 2.02x | 1.45x | 30/30 |
| materialize | 0.98x | 2.38x | 1.84x | 1.90x | 2.96x | 1.98x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.20x | 1.70x | 1.26x | 2.12x | 1.89x | 29/30 |
| perft_full_d3 | 1.17x | 1.59x | 1.84x | 1.64x | 2.21x | 1.87x | 30/30 |
| fen_parse | 1.02x | 1.50x | 1.48x | 1.21x | 1.63x | 1.19x | 30/30 |
| fen_write_native_ep | 1.00x | 3.07x | 9.85x | 2.08x | 26.42x | 2.59x | 30/30 |
| snapshot | 1.00x | 1.74x | 1.25x | 3.24x | 1.24x | 1.00x | 27/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.39x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.22x | 1.50x | 4.26x | 1.40x | 1.59x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 57.18x | 109.83x | 131.28x | 99.13x | 5/5 |
| chess960_materialize | 0.93x | 1.71x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.14x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 42.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 23.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 4.95x | 2.45x | 1.36x | 2.52x | 3.38x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.41x | 1.15x | 1.84x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.95x | 2.90x | 2.19x | 2.83x | 1.55x | 10/10 |
| replay_trusted_native | 0.98x | 0.88x | 1.64x | 1.64x | 2.27x | 2.04x | 9/10 |
| replay_checked_native | 0.99x | 2.03x | 1.17x | 8.91x | 8.57x | 9.89x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
