| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.49x | 1.65x | 1.88x | 2.32x | 2.15x | 30/30 |
| materialize | 1.01x | 1.87x | 1.58x | 1.48x | 2.12x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.15x | 1.79x | 1.53x | 2.39x | 2.35x | 30/30 |
| perft_full_d3 | 1.16x | 1.65x | 2.02x | 1.40x | 2.04x | 2.04x | 30/30 |
| fen_parse | 1.00x | 1.18x | 1.13x | 1.14x | 1.61x | 1.19x | 30/30 |
| fen_write_native_ep | 1.01x | 4.67x | 8.84x | 1.61x | 23.14x | 1.95x | 30/30 |
| snapshot | 1.00x | 1.04x | 1.06x | 1.19x | 1.05x | 0.98x | 24/30 |
| san_parse_all | 1.00x | 2.06x | N/A | 4.88x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.23x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.08x | 1.58x | 3.04x | 1.04x | 1.05x | 28/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.66x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 63.21x | 90.01x | 124.75x | 94.20x | 5/5 |
| chess960_materialize | 1.01x | 1.67x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.12x | 1.73x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 30.36x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.53x | 1.94x | 1.50x | 2.41x | 2.64x | 10/10 |
| corpus_materialize | 0.94x | 1.66x | 1.75x | 1.29x | 2.33x | 1.51x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.30x | 3.36x | 2.20x | 3.17x | 1.54x | 10/10 |
| replay_trusted_native | 1.03x | 0.68x | 1.82x | 2.07x | 2.72x | 3.39x | 8/10 |
| replay_checked_native | 1.02x | 2.65x | 1.28x | 9.36x | 8.20x | 11.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
