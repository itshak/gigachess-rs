| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.78x | 1.61x | 1.63x | 2.53x | 1.87x | 30/30 |
| materialize | 1.00x | 2.23x | 1.74x | 1.20x | 2.29x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.13x | 1.59x | 1.32x | 2.54x | 2.09x | 30/30 |
| perft_full_d3 | 1.01x | 1.28x | 1.30x | 0.99x | 1.85x | 1.60x | 26/30 |
| fen_parse | 0.75x | 1.12x | 1.22x | 0.94x | 1.73x | 1.02x | 20/30 |
| fen_write_native_ep | 1.00x | 3.97x | 9.44x | 1.81x | 27.40x | 2.03x | 30/30 |
| snapshot | 1.08x | 1.90x | 1.32x | 2.91x | 1.34x | 1.21x | 30/30 |
| san_parse_all | 0.88x | 1.93x | N/A | 3.36x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 9.81x | N/A | 2.01x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.55x | 1.03x | 3.85x | 1.15x | 1.52x | 27/30 |
| make_unmake_full | 0.83x | N/A | N/A | 1.02x | N/A | N/A | 3/6 |
| startpos | 1.27x | 1.51x | 110.48x | 169.85x | 272.57x | 181.97x | 5/5 |
| chess960_materialize | 1.01x | 1.83x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.78x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 22.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 17.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.13x | 1.95x | 1.49x | 2.70x | 2.22x | 10/10 |
| corpus_materialize | 1.01x | 2.00x | 1.78x | 1.26x | 2.93x | 1.55x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.31x | 3.13x | 2.32x | 2.99x | 1.64x | 10/10 |
| replay_trusted_native | 0.94x | 0.77x | 1.28x | 1.48x | 2.68x | 3.10x | 8/10 |
| replay_checked_native | 0.85x | 2.15x | 0.82x | 4.79x | 5.70x | 6.25x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
