| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.56x | 1.51x | 1.43x | 2.20x | 1.86x | 30/30 |
| materialize | 0.98x | 2.08x | 1.34x | 1.16x | 2.43x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.85x | 1.40x | 1.25x | 2.17x | 2.05x | 30/30 |
| perft_full_d3 | 1.00x | 1.37x | 1.48x | 1.22x | 1.95x | 2.00x | 30/30 |
| fen_parse | 0.93x | 1.31x | 1.53x | 1.51x | 2.18x | 1.51x | 30/30 |
| fen_write_native_ep | 0.96x | 3.70x | 6.99x | 1.48x | 31.58x | 2.02x | 30/30 |
| snapshot | 0.97x | 2.53x | 1.61x | 5.48x | 1.50x | 1.51x | 30/30 |
| san_parse_all | 0.98x | 1.94x | N/A | 4.56x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 11.22x | N/A | 4.01x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.67x | 1.22x | 7.75x | 1.24x | 1.69x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 89.16x | 167.04x | 233.06x | 172.56x | 5/5 |
| chess960_materialize | 1.05x | 1.59x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.65x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 37.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 23.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.48x | 1.60x | 1.07x | 2.12x | 2.21x | 9/10 |
| corpus_materialize | 1.02x | 1.60x | 1.21x | 1.05x | 2.15x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.66x | 1.97x | 1.69x | 1.92x | 1.35x | 10/10 |
| replay_trusted_native | 1.01x | 0.65x | 1.19x | 1.41x | 2.09x | 2.80x | 8/10 |
| replay_checked_native | 0.98x | 2.04x | 0.96x | 7.21x | 7.09x | 8.84x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
