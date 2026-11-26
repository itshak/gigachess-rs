| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 2.99x | 1.56x | 1.38x | 1.88x | 1.34x | 30/30 |
| materialize | 0.97x | 2.20x | 1.72x | 1.22x | 2.27x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.03x | 2.92x | 1.60x | 1.22x | 1.95x | 1.67x | 30/30 |
| perft_full_d3 | 1.01x | 1.29x | 1.52x | 1.15x | 1.75x | 1.63x | 30/30 |
| fen_parse | 0.92x | 1.43x | 1.39x | 1.12x | 1.78x | 1.20x | 30/30 |
| fen_write_native_ep | 1.01x | 2.58x | 8.98x | 1.72x | 26.54x | 2.04x | 30/30 |
| snapshot | 1.00x | 1.72x | 1.21x | 2.82x | 1.14x | 0.90x | 24/30 |
| san_parse_all | 1.01x | 2.23x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.72x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.95x | 1.20x | 4.66x | 1.14x | 1.17x | 26/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 72.75x | 108.25x | 164.01x | 113.70x | 5/5 |
| chess960_materialize | 1.03x | 1.76x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.29x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 22.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 20.40x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 3.04x | 1.65x | 1.16x | 1.98x | 1.80x | 10/10 |
| corpus_materialize | 1.02x | 2.11x | 1.84x | 1.34x | 3.05x | 1.70x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.04x | 2.92x | 2.21x | 2.83x | 1.59x | 10/10 |
| replay_trusted_native | 0.91x | 0.62x | 1.24x | 1.35x | 1.99x | 1.82x | 8/10 |
| replay_checked_native | 0.91x | 2.07x | 0.90x | 4.88x | 5.76x | 6.20x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
