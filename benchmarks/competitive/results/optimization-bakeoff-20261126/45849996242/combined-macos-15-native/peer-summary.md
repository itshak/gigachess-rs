| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.09x | 1.71x | 1.86x | 2.56x | 2.11x | 30/30 |
| materialize | 0.97x | 1.97x | 1.54x | 1.06x | 2.20x | 1.42x | 27/30 |
| perft_bulk_d4 | 1.00x | 4.16x | 1.52x | 1.30x | 1.94x | 1.90x | 30/30 |
| perft_full_d3 | 1.02x | 1.63x | 1.75x | 1.33x | 2.00x | 2.04x | 30/30 |
| fen_parse | 1.05x | 1.31x | 1.42x | 1.36x | 1.69x | 1.46x | 30/30 |
| fen_write_native_ep | 0.94x | 3.87x | 8.57x | 1.76x | 27.22x | 2.10x | 30/30 |
| snapshot | 1.01x | 1.69x | 1.13x | 2.44x | 1.14x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 1.73x | N/A | 3.68x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.13x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.07x | 1.14x | 5.33x | 1.11x | 1.26x | 26/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.33x | 67.38x | 134.60x | 157.24x | 128.00x | 5/5 |
| chess960_materialize | 0.89x | 1.82x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 3.14x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 19.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 5.90x | 1.89x | 1.12x | 2.20x | 2.40x | 9/10 |
| corpus_materialize | 1.04x | 1.51x | 1.44x | 1.16x | 1.97x | 1.22x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.84x | 2.61x | 1.85x | 2.43x | 1.37x | 10/10 |
| replay_trusted_native | 1.00x | 0.65x | 1.70x | 1.73x | 1.85x | 1.83x | 8/10 |
| replay_checked_native | 1.06x | 2.60x | 1.16x | 9.08x | 7.83x | 9.54x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
