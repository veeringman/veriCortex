mkdir -p src/api/routes
mkdir -p src/api/models
mkdir -p src/api/controllers
mkdir -p src/services/model_registry
mkdir -p src/services/trust_score
mkdir -p src/services/verifier
mkdir -p src/core
mkdir -p src/db
mkdir -p src/config
mkdir -p src/utils
mkdir -p tests
mkdir -p scripts

# Add placeholder README.md or .gitkeep files
for d in src/api/routes src/api/models src/api/controllers \
         src/services/model_registry src/services/trust_score src/services/verifier \
         src/core src/db src/config src/utils tests scripts
do
  touch $d/README.md
done

