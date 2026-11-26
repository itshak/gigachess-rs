| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.92x | 1.67x | 1.42x | 2.45x | 1.71x | 30/30 |
| materialize | 1.00x | 2.22x | 1.58x | 1.65x | 2.53x | 1.85x | 30/30 |
| perft_bulk_d4 | 0.84x | 3.27x | 1.35x | 1.01x | 2.20x | 1.61x | 27/30 |
| perft_full_d3 | 0.99x | 1.45x | 1.64x | 1.57x | 2.05x | 1.73x | 30/30 |
| fen_parse | 1.01x | 1.35x | 1.58x | 1.08x | 1.46x | 1.28x | 30/30 |
| fen_write_native_ep | 0.97x | 4.36x | 9.61x | 2.06x | 26.78x | 2.55x | 30/30 |
| snapshot | 1.67x | 1.85x | 1.16x | 4.34x | 1.16x | 1.34x | 30/30 |
| san_parse_all | 0.89x | 1.86x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.26x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.93x | 1.22x | 3.66x | 1.21x | 1.54x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.08x | N/A | N/A | 6/6 |
| startpos | 1.34x | 1.11x | 92.05x | 153.33x | 193.21x | 180.65x | 5/5 |
| chess960_materialize | 1.02x | 1.70x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.39x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.92x | 37.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.83x | 16.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.66x | 4.26x | 1.60x | 0.88x | 1.93x | 1.74x | 8/10 |
| corpus_materialize | 0.95x | 1.37x | 1.25x | 1.04x | 1.66x | 1.15x | 9/10 |
| corpus_uci_sorted | 0.99x | 2.27x | 2.94x | 2.20x | 2.82x | 1.56x | 10/10 |
| replay_trusted_native | 1.01x | 0.83x | 1.53x | 1.45x | 2.05x | 2.45x | 9/10 |
| replay_checked_native | 1.02x | 2.10x | 1.17x | 8.89x | 8.62x | 10.32x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
