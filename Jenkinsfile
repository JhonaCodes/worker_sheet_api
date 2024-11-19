pipeline {
    agent any

    environment {
        DOCKER_REGISTRY = 'jhonacode'
        APP_NAME = 'worker-sheet-api'
    }

    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }

        stage('Security Scan') {
            steps {
                sh 'cargo audit'
            }
        }

        stage('Build Docker Image') {
            steps {
                script {
                    docker.build("${DOCKER_REGISTRY}/${APP_NAME}:${BUILD_NUMBER}")
                }
            }
        }

        stage('Deploy Development') {
            when { branch 'develop' }
            steps {
                sh '''
                    export ENV_FILE=dev.env
                    docker-compose --profile dev down
                    docker-compose --profile dev up -d
                '''
            }
        }
    }

    post {
        always {
            cleanWs()
        }
        success {
            emailext (
                subject: "Build Exitoso: ${env.JOB_NAME} [${env.BUILD_NUMBER}]",
                body: "El build se completó exitosamente.",
                to: "{EMAIL}"
            )
        }
        failure {
            emailext (
                subject: "Build Fallido: ${env.JOB_NAME} [${env.BUILD_NUMBER}]",
                body: "El build falló. Revisa los logs.",
                to: "{EMAIL}"
            )
        }
    }
}