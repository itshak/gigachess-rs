| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.86x | 1.38x | 1.36x | 1.75x | 1.60x | 30/30 |
| materialize | 1.08x | 1.95x | 1.54x | 1.01x | 2.33x | 1.35x | 27/30 |
| perft_bulk_d4 | 1.06x | 3.57x | 1.37x | 1.10x | 1.70x | 1.55x | 29/30 |
| perft_full_d3 | 1.16x | 1.54x | 1.76x | 1.34x | 1.94x | 2.11x | 30/30 |
| fen_parse | 1.12x | 1.20x | 1.27x | 1.30x | 1.67x | 1.34x | 30/30 |
| fen_write_native_ep | 1.05x | 4.41x | 9.42x | 1.85x | 28.93x | 2.35x | 30/30 |
| snapshot | 0.93x | 1.64x | 1.13x | 2.13x | 1.03x | 0.93x | 26/30 |
| san_parse_all | 1.02x | 1.87x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 1.08x | 10.91x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.17x | 1.20x | 1.25x | 5.70x | 1.16x | 1.29x | 26/30 |
| make_unmake_full | 1.13x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 0.88x | 1.42x | 67.17x | 116.09x | 156.60x | 111.75x | 5/5 |
| chess960_materialize | 0.97x | 1.68x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.08x | 2.57x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 39.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.83x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.96x | 17.80x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 4.25x | 1.39x | 0.89x | 1.83x | 1.80x | 8/10 |
| corpus_materialize | 1.10x | 1.36x | 1.37x | 1.14x | 1.91x | 1.46x | 10/10 |
| corpus_uci_sorted | 0.85x | 1.91x | 2.46x | 1.99x | 2.71x | 1.47x | 10/10 |
| replay_trusted_native | 0.87x | 0.64x | 1.70x | 1.70x | 1.54x | 2.08x | 8/10 |
| replay_checked_native | 1.15x | 2.44x | 1.12x | 8.32x | 7.42x | 9.68x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
