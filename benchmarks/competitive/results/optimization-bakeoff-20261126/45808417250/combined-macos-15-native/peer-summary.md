| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.07x | 4.43x | 1.48x | 1.54x | 1.99x | 1.77x | 30/30 |
| materialize | 1.13x | 1.97x | 1.49x | 1.03x | 2.32x | 1.31x | 28/30 |
| perft_bulk_d4 | 1.06x | 4.39x | 1.60x | 1.30x | 2.03x | 1.93x | 30/30 |
| perft_full_d3 | 1.08x | 1.52x | 1.75x | 1.37x | 2.03x | 2.00x | 30/30 |
| fen_parse | 1.18x | 1.33x | 1.40x | 1.43x | 1.83x | 1.48x | 30/30 |
| fen_write_native_ep | 1.06x | 4.01x | 8.65x | 1.84x | 28.45x | 2.30x | 30/30 |
| snapshot | 0.98x | 1.55x | 1.02x | 2.18x | 1.06x | 0.97x | 22/30 |
| san_parse_all | 1.08x | 1.81x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.15x | 12.52x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 0.99x | 1.09x | 4.84x | 0.97x | 1.18x | 22/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.04x | N/A | N/A | 3/6 |
| startpos | 1.03x | 1.33x | 113.01x | 174.46x | 220.59x | 162.70x | 5/5 |
| chess960_materialize | 0.94x | 1.94x | 1.79x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.80x | 3.41x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 38.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.90x | 0.88x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.95x | 16.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 6.08x | 1.95x | 1.39x | 2.74x | 2.63x | 10/10 |
| corpus_materialize | 1.03x | 1.54x | 1.38x | 1.18x | 1.81x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.91x | 2.76x | 1.82x | 2.72x | 1.55x | 10/10 |
| replay_trusted_native | 0.89x | 0.52x | 1.31x | 1.32x | 1.44x | 1.64x | 8/10 |
| replay_checked_native | 0.89x | 2.02x | 0.94x | 7.27x | 6.75x | 7.95x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
