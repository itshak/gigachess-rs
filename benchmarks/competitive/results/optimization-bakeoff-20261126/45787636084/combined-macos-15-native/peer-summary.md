| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 4.62x | 1.43x | 1.65x | 2.03x | 1.83x | 30/30 |
| materialize | 0.96x | 2.08x | 1.52x | 1.04x | 2.24x | 1.33x | 28/30 |
| perft_bulk_d4 | 0.67x | 3.55x | 1.27x | 1.01x | 1.57x | 1.56x | 26/30 |
| perft_full_d3 | 0.90x | 1.53x | 1.78x | 1.34x | 1.96x | 2.12x | 29/30 |
| fen_parse | 0.90x | 1.26x | 1.33x | 1.32x | 1.72x | 1.45x | 30/30 |
| fen_write_native_ep | 0.92x | 4.55x | 8.97x | 1.75x | 28.55x | 2.27x | 30/30 |
| snapshot | 0.88x | 1.68x | 1.17x | 2.61x | 1.15x | 0.92x | 24/30 |
| san_parse_all | 0.85x | 1.85x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 0.89x | 11.35x | N/A | 2.60x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.00x | 1.02x | 4.90x | 1.06x | 1.07x | 20/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 0.94x | 1.37x | 78.30x | 144.96x | 180.84x | 154.14x | 5/5 |
| chess960_materialize | 0.83x | 1.39x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.70x | 2.82x | 1.23x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 39.86x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 20.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.75x | 4.81x | 1.48x | 0.96x | 1.81x | 2.24x | 9/10 |
| corpus_materialize | 0.99x | 1.53x | 1.33x | 1.29x | 1.90x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.80x | 1.81x | 2.41x | 1.92x | 2.37x | 1.36x | 10/10 |
| replay_trusted_native | 0.94x | 0.61x | 1.41x | 1.47x | 1.61x | 1.82x | 8/10 |
| replay_checked_native | 0.92x | 2.21x | 0.93x | 6.93x | 6.58x | 8.64x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
