from flask import Flask, jsonify

app = Flask(__name__)

dokumenty = [
    {
        'id': 1,
        'title': u'First message',
        'description': u'Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industrys standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.', 
        'done': False
    },
    {
        'id': 2,
        'title': u'Second message',
        'description': u'It has survived not only five centuries.', 
        'done': False
    }
]

@app.route('/dokumenty', methods=['GET'])
def get_tasks():
    return jsonify({'dokumenty': dokumenty})

if __name__ == '__main__':
    app.run(debug=True)