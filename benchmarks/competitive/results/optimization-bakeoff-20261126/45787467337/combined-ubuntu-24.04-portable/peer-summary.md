| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.93x | 1.50x | 1.37x | 1.91x | 1.35x | 30/30 |
| materialize | 1.00x | 1.97x | 1.76x | 1.26x | 2.35x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.87x | 1.56x | 1.22x | 1.96x | 1.70x | 30/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.70x | 1.40x | 2.05x | 1.85x | 30/30 |
| fen_parse | 0.99x | 1.42x | 1.53x | 1.18x | 2.23x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 2.93x | 9.28x | 1.68x | 27.83x | 2.00x | 30/30 |
| snapshot | 1.00x | 1.71x | 1.20x | 3.05x | 1.01x | 1.06x | 29/30 |
| san_parse_all | 1.04x | 2.13x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.46x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.07x | 1.36x | 3.99x | 1.25x | 1.20x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.29x | 64.46x | 112.14x | 187.70x | 131.25x | 5/5 |
| chess960_materialize | 0.98x | 1.46x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.04x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 26.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.93x | 19.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.90x | 1.94x | 1.29x | 2.14x | 2.59x | 10/10 |
| corpus_materialize | 1.01x | 1.64x | 1.65x | 1.20x | 2.12x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.24x | 3.21x | 2.45x | 3.10x | 1.82x | 10/10 |
| replay_trusted_native | 1.04x | 0.74x | 1.41x | 1.66x | 2.18x | 2.03x | 8/10 |
| replay_checked_native | 1.02x | 2.26x | 1.08x | 7.63x | 7.57x | 8.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
