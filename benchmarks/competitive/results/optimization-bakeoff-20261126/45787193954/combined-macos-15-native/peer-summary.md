| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.76x | 1.42x | 1.50x | 1.90x | 1.76x | 30/30 |
| materialize | 0.96x | 1.85x | 1.36x | 0.93x | 2.14x | 1.19x | 24/30 |
| perft_bulk_d4 | 1.14x | 4.63x | 1.70x | 1.52x | 2.15x | 2.24x | 30/30 |
| perft_full_d3 | 1.04x | 1.62x | 1.71x | 1.47x | 1.95x | 2.05x | 30/30 |
| fen_parse | 0.88x | 1.25x | 1.28x | 1.25x | 1.68x | 1.35x | 27/30 |
| fen_write_native_ep | 0.88x | 3.90x | 8.72x | 1.67x | 26.42x | 2.22x | 30/30 |
| snapshot | 0.96x | 1.56x | 1.17x | 2.51x | 1.08x | 1.04x | 24/30 |
| san_parse_all | 0.80x | 1.82x | N/A | 4.73x | N/A | N/A | 12/12 |
| san_render_native | 0.86x | 11.94x | N/A | 2.61x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 0.95x | 1.13x | 4.94x | 1.05x | 1.10x | 23/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.06x | 1.16x | 63.29x | 132.87x | 152.61x | 118.40x | 5/5 |
| chess960_materialize | 0.96x | 1.55x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 3.11x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.15x | 40.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.24x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 18.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.92x | 5.55x | 2.02x | 1.05x | 2.25x | 2.21x | 10/10 |
| corpus_materialize | 1.19x | 1.53x | 1.41x | 1.11x | 1.95x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.03x | 1.80x | 2.65x | 1.85x | 2.49x | 1.32x | 10/10 |
| replay_trusted_native | 1.05x | 0.61x | 1.72x | 1.60x | 1.71x | 1.62x | 8/10 |
| replay_checked_native | 1.04x | 2.07x | 0.97x | 7.07x | 6.53x | 7.29x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
