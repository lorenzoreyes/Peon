from flask import Flask
from flask_jsonpify import jsonpify
import pandas as pd 
app = Flask(__name__)

from products import products

@app.route('/market')
def market():
    excel = pd.read_excel('Central Bank Report.xlsx')
    excel.index = excel['Unnamed: 0'].to_list()
    del excel['Unnamed: 0']
    excel = excel[['tc_oficial', 'solidario', 'Cable Apple', 'FX Fundamental',
       'Monetarista Blue','AL30']].tail(20)
    excel.columns = ['Oficial','Solidario','Cable','Fx-Fundamental','Monetarista','Bono-al30']
    excel = excel.drop_duplicates()
    excel = excel.to_json(orient='table')
    return excel 

if __name__=='__main__':
    app.run(debug=True)
