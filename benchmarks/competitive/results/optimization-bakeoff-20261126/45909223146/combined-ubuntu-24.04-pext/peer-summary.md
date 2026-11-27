| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.62x | 1.94x | 1.67x | 3.02x | 1.98x | 30/30 |
| materialize | 1.04x | 2.35x | 1.69x | 1.76x | 2.80x | 2.03x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.98x | 1.58x | 1.24x | 2.63x | 1.94x | 29/30 |
| perft_full_d3 | 0.99x | 1.33x | 1.53x | 1.50x | 1.99x | 1.62x | 30/30 |
| fen_parse | 0.94x | 1.20x | 1.24x | 1.08x | 1.38x | 1.19x | 29/30 |
| fen_write_native_ep | 0.97x | 4.85x | 9.73x | 1.82x | 28.02x | 2.15x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.66x | 1.00x | 1.33x | 24/30 |
| san_parse_all | 1.02x | 2.06x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.06x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.16x | 1.35x | 4.14x | 1.44x | 1.65x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.00x | 78.48x | 146.64x | 175.05x | 155.49x | 4/5 |
| chess960_materialize | 1.00x | 1.77x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.52x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 40.14x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.82x | 20.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.19x | 6.41x | 2.35x | 1.33x | 2.95x | 2.68x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.09x | 1.74x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.27x | 3.01x | 2.19x | 2.93x | 1.52x | 10/10 |
| replay_trusted_native | 0.96x | 0.85x | 1.53x | 1.55x | 2.15x | 2.48x | 9/10 |
| replay_checked_native | 1.01x | 2.21x | 1.17x | 9.38x | 8.94x | 10.50x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
