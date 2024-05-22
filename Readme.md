```bash
./reBOrN \
  --input=jars/forge-1.7.10-10.13.4.1614-1.7.10-srg.jar \
  --input=jars/forge-1.7.10-10.13.4.1614-1.7.10-decomp.jar \
  --input=jars/Thaumcraft-1.7.10-4.2.3.5.jar \
  --mappings stable_12 \
  --extra-mappings 'methods:https://demo.web.veritaris.me/static/methods.csv' \
  --extra-mappings 'fields:https://demo.web.veritaris.me/static/fields.csv' \
  --extra-mappings 'params:https://demo.web.veritaris.me/static/params.csv' \
  --extra-mappings 'params:file:///Users/veritaris/methods.csv' \
  --extra-mappings 'params:p_72607_1_=msg;p_72601_1_=socket'
```