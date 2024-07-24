echo "-- healthcheck --"
curl localhost:8000/api/healthcheck
echo "\n-- add job 1 --"
curl -X POST http://localhost:8000/api/add-job \
     -H "Content-Type: application/json" \
     -d '{"input":"Example input", "user":"tom"}'
echo "\n-- add job 2 --"
curl -X POST http://localhost:8000/api/add-job \
     -H "Content-Type: application/json" \
     -d '{"input":"More input!", "user":"bob"}'
echo "\n-- view jobs --"
curl http://localhost:8000/api/view-jobs
echo ""
